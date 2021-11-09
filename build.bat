@echo off
nimble install --depsOnly
nim c -d:release -d:strip --opt:size --outdir:bin cli/*.nim

upx bin/*.exe