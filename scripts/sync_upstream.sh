#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
用法:
  scripts/sync_upstream.sh [选项]

默认流程:
  1) git fetch upstream/origin
  2) 切到 custom/main 并 fast-forward 拉取 origin/custom/main
  3) 新建同步分支 sync/upstream-YYYYMMDD
  4) 合并 upstream/main

选项:
  -u <name>  upstream 远程名（默认: upstream）
  -o <name>  origin 远程名（默认: origin）
  -c <name>  自定义主分支（默认: custom/main）
  -s <name>  upstream 分支（默认: main）
  -b <name>  同步分支名（默认: sync/upstream-YYYYMMDD）
  -h         显示帮助
EOF
}

die() {
  echo "错误: $*" >&2
  exit 1
}

upstream_remote="upstream"
origin_remote="origin"
custom_branch="custom/main"
upstream_branch="main"
sync_branch=""

while getopts ":u:o:c:s:b:h" opt; do
  case "$opt" in
    u) upstream_remote="$OPTARG" ;;
    o) origin_remote="$OPTARG" ;;
    c) custom_branch="$OPTARG" ;;
    s) upstream_branch="$OPTARG" ;;
    b) sync_branch="$OPTARG" ;;
    h)
      usage
      exit 0
      ;;
    :)
      die "选项 -$OPTARG 需要参数"
      ;;
    \?)
      die "未知选项: -$OPTARG"
      ;;
  esac
done

shift $((OPTIND - 1))
if [[ $# -ne 0 ]]; then
  usage
  die "不支持位置参数"
fi

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "当前目录不是 Git 仓库"

if [[ -n "$(git status --porcelain)" ]]; then
  die "工作区不干净，请先提交或暂存改动后再执行"
fi

git remote get-url "$upstream_remote" >/dev/null 2>&1 || die "远程不存在: $upstream_remote"
git remote get-url "$origin_remote" >/dev/null 2>&1 || die "远程不存在: $origin_remote"

if [[ -z "$sync_branch" ]]; then
  sync_branch="sync/upstream-$(date +%Y%m%d)"
fi

base_sync_branch="$sync_branch"
suffix=0
while git show-ref --verify --quiet "refs/heads/$sync_branch" \
  || git ls-remote --exit-code --heads "$origin_remote" "$sync_branch" >/dev/null 2>&1; do
  suffix=$((suffix + 1))
  sync_branch="${base_sync_branch}-$(date +%H%M%S)-$suffix"
done

echo "[1/6] 拉取远程引用..."
git fetch "$upstream_remote" --prune --tags
git fetch "$origin_remote" --prune

echo "[2/6] 校验远程分支..."
origin_custom_ref="refs/remotes/$origin_remote/$custom_branch"
upstream_ref="refs/remotes/$upstream_remote/$upstream_branch"
git show-ref --verify --quiet "$origin_custom_ref" || die "远程分支不存在: $origin_remote/$custom_branch"
git show-ref --verify --quiet "$upstream_ref" || die "远程分支不存在: $upstream_remote/$upstream_branch"

echo "[3/6] 切换到 $custom_branch ..."
git switch "$custom_branch"

echo "[4/6] 快进更新 $custom_branch <- $origin_remote/$custom_branch ..."
git pull --ff-only "$origin_remote" "$custom_branch"

echo "[5/6] 创建同步分支 $sync_branch ..."
git switch -c "$sync_branch"

echo "[6/6] 合并 $upstream_remote/$upstream_branch ..."
if ! git merge --no-ff "$upstream_remote/$upstream_branch" -m "chore(sync): merge $upstream_remote/$upstream_branch"; then
  cat <<'EOF' >&2

合并发生冲突，请手动处理后继续：
  1) 解决冲突并执行 git add <files>
  2) 完成合并提交: git commit
  3) 需要放弃本次合并: git merge --abort
EOF
  exit 1
fi

cat <<EOF

同步完成。
当前分支: $sync_branch

建议下一步:
  1) 运行项目测试
  2) 推送分支: git push -u $origin_remote $sync_branch
  3) 发起 PR: $sync_branch -> $custom_branch
EOF
