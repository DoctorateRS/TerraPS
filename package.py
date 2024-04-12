import toml
import py7zr

cargo = toml.load("Cargo.toml")
version = cargo["package"]["version"]

with py7zr.SevenZipFile(f"packaged/TerraPS_{version}.7z", "w") as archive:
    archive.write("target/release/terra-ps.exe", "TerraPS.exe")
    archive.writeall("data", "data")
    archive.writeall("config", "config")
    archive.write("README.md", "README.md")
    archive.write("LICENSE", "LICENSE")
