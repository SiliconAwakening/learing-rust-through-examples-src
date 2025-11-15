# Git 工作流程指南

本文档说明如何使用 Git 管理本项目的代码。

## 基本工作流程

### 1. 查看当前状态

```bash
git status
```

这会显示：
- 当前分支
- 已修改但未暂存的文件
- 已暂存但未提交的文件
- 未跟踪的文件

### 2. 添加文件到暂存区

```bash
# 添加所有更改
git add .

# 添加特定文件
git add chapter02/src/calculator/parser.rs

# 添加特定目录
git add chapter02/
```

### 3. 提交更改

```bash
# 提交暂存的更改
git commit -m "feat: 添加 Unicode 数学符号支持"

# 提交并附带详细说明
git commit -m "feat: 实现表达式求值器

- 添加词法分析器
- 添加语法分析器
- 添加求值器
- 添加 46 个测试用例"
```

### 4. 推送到远程仓库

```bash
# 首次推送（设置上游分支）
git push -u origin main

# 后续推送
git push
```

## 提交信息规范

使用约定式提交（Conventional Commits）格式：

### 提交类型

- `feat:` - 新功能
- `fix:` - 修复 bug
- `docs:` - 文档更新
- `style:` - 代码格式调整（不影响功能）
- `refactor:` - 重构代码
- `test:` - 添加或修改测试
- `chore:` - 构建过程或辅助工具的变动

### 示例

```bash
# 新功能
git commit -m "feat: 添加平方根符号 √ 支持"

# 修复 bug
git commit -m "fix: 修复除零错误处理"

# 文档更新
git commit -m "docs: 更新 README 添加 Unicode 符号说明"

# 重构
git commit -m "refactor: 优化解析器性能"

# 测试
git commit -m "test: 添加 Unicode 符号测试用例"

# 构建工具
git commit -m "chore: 更新依赖版本"
```

## 常用命令

### 查看提交历史

```bash
# 查看提交日志
git log

# 简洁的一行显示
git log --oneline

# 图形化显示分支
git log --oneline --graph --all

# 查看最近 5 次提交
git log -5
```

### 查看更改

```bash
# 查看工作区的更改（未暂存）
git diff

# 查看暂存区的更改
git diff --staged

# 查看特定文件的更改
git diff chapter02/src/calculator/parser.rs
```

### 撤销更改

```bash
# 撤销工作区的更改（危险！会丢失更改）
git checkout -- <file>

# 撤销暂存（保留工作区更改）
git reset HEAD <file>

# 撤销最后一次提交（保留更改）
git reset --soft HEAD~1

# 撤销最后一次提交（不保留更改，危险！）
git reset --hard HEAD~1
```

### 分支管理

```bash
# 查看所有分支
git branch -a

# 创建新分支
git branch feature/new-feature

# 切换分支
git checkout feature/new-feature

# 创建并切换到新分支
git checkout -b feature/new-feature

# 合并分支
git checkout main
git merge feature/new-feature

# 删除分支
git branch -d feature/new-feature
```

## 推荐工作流程

### 开发新功能

```bash
# 1. 确保 main 分支是最新的
git checkout main
git pull

# 2. 创建功能分支
git checkout -b feature/unicode-symbols

# 3. 开发并多次提交
git add .
git commit -m "feat: 添加 Unicode tokenizer"
git add .
git commit -m "feat: 实现 Unicode 解析逻辑"
git add .
git commit -m "test: 添加 Unicode 测试用例"

# 4. 推送功能分支
git push -u origin feature/unicode-symbols

# 5. 合并到 main（在 GitHub 上创建 Pull Request，或本地合并）
git checkout main
git merge feature/unicode-symbols
git push

# 6. 删除功能分支（可选）
git branch -d feature/unicode-symbols
git push origin --delete feature/unicode-symbols
```

### 修复 Bug

```bash
# 1. 创建修复分支
git checkout -b fix/division-by-zero

# 2. 修复并提交
git add .
git commit -m "fix: 修复除零错误处理"

# 3. 推送并合并
git push -u origin fix/division-by-zero
git checkout main
git merge fix/division-by-zero
git push
```

## 同步远程仓库

```bash
# 获取远程更新（不合并）
git fetch origin

# 拉取并合并远程更新
git pull

# 拉取并变基（推荐，保持提交历史整洁）
git pull --rebase
```

## 常见问题解决

### 推送被拒绝（远程有新提交）

```bash
# 方法 1：拉取并合并
git pull
git push

# 方法 2：拉取并变基（推荐）
git pull --rebase
git push
```

### 合并冲突

```bash
# 1. 拉取时发生冲突
git pull
# CONFLICT (content): Merge conflict in file.rs

# 2. 手动解决冲突（编辑文件，删除冲突标记）
# <<<<<<< HEAD
# 你的更改
# =======
# 远程的更改
# >>>>>>> origin/main

# 3. 添加已解决的文件
git add file.rs

# 4. 完成合并
git commit -m "merge: 解决合并冲突"
git push
```

### 意外提交了不该提交的文件

```bash
# 从暂存区移除（保留文件）
git reset HEAD <file>

# 从最后一次提交中移除（文件已提交）
git reset --soft HEAD~1
git reset HEAD <file>
git commit -m "正确的提交信息"
```

### 修改最后一次提交信息

```bash
# 修改提交信息（未推送）
git commit --amend -m "新的提交信息"

# 添加遗漏的文件到最后一次提交
git add forgotten_file.rs
git commit --amend --no-edit
```

## 忽略文件

编辑 `.gitignore` 文件：

```gitignore
# Rust
target/
Cargo.lock  # 对于库项目
**/*.rs.bk
*.pdb

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# macOS
.DS_Store

# 临时文件
*.tmp
*.log
```

## 标签管理

```bash
# 创建标签
git tag v1.0.0
git tag -a v1.0.0 -m "版本 1.0.0 - 表达式计算器首个发布版"

# 推送标签
git push origin v1.0.0

# 推送所有标签
git push --tags

# 查看标签
git tag

# 删除标签
git tag -d v1.0.0
git push origin --delete v1.0.0
```

## 最佳实践

### 提交频率
- ✅ 频繁提交小的逻辑单元
- ✅ 每个提交应该是一个完整的功能或修复
- ❌ 避免一次提交包含多个不相关的更改

### 提交信息
- ✅ 使用现在时态（"添加功能" 而不是 "添加了功能"）
- ✅ 首行不超过 50 字符
- ✅ 如需详细说明，空一行后添加详细内容
- ❌ 避免无意义的提交信息（如 "更新"、"修改"）

### 分支管理
- ✅ main 分支保持稳定可用
- ✅ 功能开发使用独立分支
- ✅ 合并前确保测试通过
- ❌ 避免直接在 main 分支开发

### 代码审查
- ✅ 使用 Pull Request 进行代码审查
- ✅ 合并前运行所有测试
- ✅ 保持提交历史整洁

## 快速参考

```bash
# 日常工作流程
git status                    # 查看状态
git add .                     # 添加所有更改
git commit -m "feat: xxx"     # 提交
git push                      # 推送

# 查看历史
git log --oneline            # 简洁日志
git diff                     # 查看更改

# 分支操作
git branch                   # 查看分支
git checkout -b feature/xxx  # 创建并切换分支
git merge feature/xxx        # 合并分支

# 同步
git pull                     # 拉取远程更新
git push                     # 推送本地提交

# 撤销
git reset HEAD <file>        # 撤销暂存
git checkout -- <file>       # 撤销工作区更改
```

## 相关资源

- [Git 官方文档](https://git-scm.com/doc)
- [Pro Git 书籍（中文版）](https://git-scm.com/book/zh/v2)
- [约定式提交规范](https://www.conventionalcommits.org/zh-hans/)
- [Git 可视化学习](https://learngitbranching.js.org/?locale=zh_CN)

## 获取帮助

```bash
# 查看命令帮助
git help <command>
git <command> --help

# 示例
git help commit
git add --help
```
