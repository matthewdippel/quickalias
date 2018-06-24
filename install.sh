#/usr/bash!
INSTALL_DIR=$1
red=`tput setaf 1`
green=`tput setaf 2`
reset=`tput sgr0`
function display_error(){
    echo "$1" | while read line ; do
        echo "    ${red}$line${reset}"
    done
}
function display_good(){
    echo "$1" | while read line ; do
        echo "    ${green}$line${reset}"
    done
}
if [ ! -d "$INSTALL_DIR" ] ; then
    echo "$INSTALL_DIR doesn't exist, making..."
    err="$(mkdir $INSTALL_DIR 2>&1)"
    OUT=$?
    if [ $OUT -ne 0 ]; then
        echo "Couldn't create directory $INSTALL_DIR due to error:"
        display_error "$err"
        echo "Halting installation."
        exit 1
    fi
fi
echo "Installing to $INSTALL_DIR"
echo "Building Rust project in Cargo..."
err="$(cargo build --release -q 2>&1)"
OUT=$?
if [ $OUT -ne 0 ]; then
    echo "Couldn't build the Rust project due to error:"
    display_error "$err"
    echo "Please open an issue and copy / paste the above error."
    echo "Halting installation."
    exit 1
fi
err="$(cp target/release/quick_alias ${INSTALL_DIR} 2>&1)"
OUT=$?
if [ $OUT -ne 0 ]; then
    echo "Couldn't move compiled binary to ${INSTALL_DIR} due to error"
    display_error "$err"
    echo "Halting installation."
    exit 1
fi
display_good "Installation complete"
