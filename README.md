# 開発環境ステータスダッシュボード

## 使用技術
- Rust
- htmx
- Tailwind

開発環境ではTailwindを編集した際に再生性が必要
```
./tools/tailwindcss -i ./assets/tailwind.css -o ./static/app.css --watch
```