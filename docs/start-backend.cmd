@echo off
echo Starting Kankan Backend Server...
cd kankan
call mvnw.cmd spring-boot:run
pause