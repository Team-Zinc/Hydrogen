import hydrogen

def build_open_ssl():
    if hydrogen.system() == "windows":
        hydrogen.execute([
            "perl ./Configure",
            "nmake",
            "nmake install"
        ])
    else:
        hydrogen.execute([
            "./Configure",
            "make",
            "make install"
        ])

if hydrogen.code.exists("openssl") == False:
    build_open_ssl()
