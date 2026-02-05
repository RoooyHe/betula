@echo off
echo ========================================
echo    Betula Kanban 看板管理应用
echo ========================================
echo.
echo 正在启动项目...
echo.

echo [1/3] 检查环境...
echo 检查 Java 环境...
java -version >nul 2>&1
if %errorlevel% neq 0 (
    echo 错误: 未找到 Java 环境，请确保已安装 JDK 25+ 并配置 JAVA_HOME
    echo 请访问: https://www.oracle.com/java/technologies/downloads/
    pause
    exit /b 1
)

echo 检查 Rust 环境...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 错误: 未找到 Rust 环境，请安装 Rust
    echo 请访问: https://rustup.rs/
    pause
    exit /b 1
)

echo [2/3] 启动后端服务 (Kankan)...
cd kankan
start "Kankan Backend" cmd /k "echo 启动 Kankan 后端服务... && mvnw.cmd spring-boot:run"
cd ..

echo [3/3] 等待后端服务启动...
echo 等待 10 秒让后端服务完全启动...
timeout /t 10 /nobreak > nul

echo 启动前端应用 (Betula)...
echo 请确保后端服务已在 http://localhost:8911 正常运行
echo.
cargo run

pause