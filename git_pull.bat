@echo off
:: 自动化 Git 操作脚本
:: 用于执行 git pull, git add, git commit 和 git push

:: 捕获 Ctrl+C 中断
:: 禁止 Ctrl+C 中断
echo Press Ctrl+C to abort at any point.

:: 开始执行命令
echo Starting git pull...
git pull
if %ERRORLEVEL% neq 0 (
    echo "git pull failed, stopping..."
    pause
    exit /b %ERRORLEVEL%
)
echo Git pull completed successfully.
pause

echo Starting git add...
git add .
if %ERRORLEVEL% neq 0 (
    echo "git add failed, stopping..."
    pause
    exit /b %ERRORLEVEL%
)
echo Git add completed successfully.
pause

echo Starting git commit...
git commit -m "Your commit message"
if %ERRORLEVEL% neq 0 (
    echo "git commit failed, stopping..."
    pause
    exit /b %ERRORLEVEL%
)
echo Git commit completed successfully.
pause

echo Starting git push...
git push
if %ERRORLEVEL% neq 0 (
    echo "git push failed, stopping..."
    pause
    exit /b %ERRORLEVEL%
)
echo Git push completed successfully.
pause

:: 保持窗口打开，不需要按任意键
cmd /k echo "All Git commands executed successfully! Press Ctrl+C to close."
