nimble install --depsOnly
nim c -d:release -d:strip --opt:size *.nim
upx *.exe