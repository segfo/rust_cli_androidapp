# rust_cli_androidapp
Android上で動作するコンソールアプリ(aarch64/armv7)

# ビルド方法
## ビルド環境
OS: Ubuntu 16.04.3 LTS (Windows 10 BoW)

1. ここからNDKを落とします。（SDKは不要）
https://developer.android.com/ndk/downloads/index.html?hl=ja

2. android-ndk-r14b-linux-x86_64.zip をC:ドライブ直下にそのまま展開します。

3. ディレクトリ構造を確認します。
OKの例  
C:  
┣android-ndk-r14b  
┃┣build  
┃┣platforms  
┃┃┣android-21  
┃┃┃┗arch-arm64  
┃┃┣andorid-19  
┃┃┃┗arch-arm  
┃┣prebuilt  
┃┣python-packages  
┃┣shader-tools  
┃┣simpleperf  
┃┣sources  
┃┣sysroot  
┃┣toolchains  
┃┃┗arm-linux-androideabi-4.9  
┃┃┃ ・・・・　省略  
  
こんなふうになってればOK  
  
NGの例  
C:  
┣android-ndk-r14b-linux-x86_64 <- 不要  
┃┣android-ndk-r14b  
┃┃┣build  
┃┃┣platforms  
┃┃┃┣android-21  
・・・ これはNG  
  
ここまでセットしたら後は4番の手順  
  
4. cargo build  
