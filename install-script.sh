# go to the user root directory
cd ~

# `Attempting to install plum...`
echo "Attempting to install plum..."

# Clone the full repo.
git clone https://github.com/cantoramann/plum.git

# Move the cli directory to user/.plumcli/
mv plum/cli ~/.plumcli

# cd into .plumcli directory
cd ~/.plumcli

# `Attempting to configure plum...`
echo "Attempting to configure plum..."

# .plumcli directory is a Rust project, so build it.
cargo build --release

# Add the .plumcli directory to the PATH to print the below message.
# # >>> .plumcli initialize >>>
# # !! Contents within this block are managed by the user !!
# PLUM_DIR="$HOME/.plumcli/target/release"
# if [ -d "$PLUM_DIR" ]; then
#     export PATH="$PLUM_DIR:$PATH"
# fi
# # <<< .plumcli initialize <<<
echo "# >>> .plumcli initialize >>>
# !! Contents within this block are managed by the user !!
PLUM_DIR=\"$HOME/.plumcli/target/release\"
if [ -d \"\$PLUM_DIR\" ]; then
    export PATH=\"\$PLUM_DIR:\$PATH\"
fi
# <<< .plumcli initialize <<<" >> ~/.bashrc


# `Plum installed successfully! It is in the hidden directory ~/.plum`
echo "Plum installed successfully! It is in the hidden directory ~/.plum"