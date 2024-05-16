# 初始化

- cargo generate tyr-rust-bootcamp/template
- update `build.yml` master -> main
- run `pre-commit install`
- git remote add origin https://github.com/kindywu/03-safe-server-error.git
- git branch -M main
- git add .
- git commit -a -m "init"
- git push -u origin main

# 调试

- $env:RUST_LOG="debug"; cargo run --example web
