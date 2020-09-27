import hydrogen

def build_sodium():
    if hydrogen.system() == "windows":
        hydrogen.build.msbuild("libsodium")
    else:
        hydrogen.build.autotools()

if hydrogen.code.exists("openssl") == False:
    build_sodium();
