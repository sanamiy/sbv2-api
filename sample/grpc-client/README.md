# クライアント側のコードを以下のコマンドで生成済み

```
npx proto-loader-gen-types ../../sbv2_grpc/proto/tts.proto -O ./proto --grpcLib=@grpc/grpc-js
```

# 実行時

```sh
cargo run --bin sbv2_grpc
```

```sh
cd sample/grpc-client
ts-node test.ts
```
