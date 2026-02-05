@echo off
echo ========================================
echo    Betula 开发环境检查
echo ========================================
echo.

echo [1/4] 检查 Java 环境...
java -version 2>&1 | findstr "version" >nul
if %errorlevel% equ 0 (
    echo ✓ Java 已安装
    java -version
) else (
    echo ✗ Java 未安装或未配置
    echo   请安装 JDK 25+ 并配置 JAVA_HOME 环境变量
    echo   下载地址: https://www.oracle.com/java/technologies/downloads/
)
echo.

echo [2/4] 检查 Maven...
cd kankan
if exist "mvnw.cmd" (
    echo ✓ Maven Wrapper 已就绪
) else (
    echo ✗ Maven Wrapper 未找到
)
cd ..
echo.

echo [3/4] 检查 Rust 环境...
cargo --version >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ Rust 已安装
    cargo --version
    rustc --version
) else (
    echo ✗ Rust 未安装
    echo   请访问 https://rustup.rs/ 安装 Rust
)
echo.

echo [4/4] 检查项目依赖...
echo 检查 Rust 项目依赖...
cargo check >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ Rust 项目依赖正常
) else (
    echo ⚠ Rust 项目依赖需要更新，运行 'cargo check' 查看详情
)

echo.
echo 检查完成！
echo.
echo 如果所有检查都通过，可以运行以下命令启动项目：
echo   start-project.cmd     # 启动完整项目
echo   start-backend.cmd     # 仅启动后端
echo   start-frontend.cmd    # 仅启动前端
echo.
pause