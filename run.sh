[ -z "${IMAGEPATH}" ] && IMAGEPATH="$(dirname -- "$0")/image.ppm"

cargo run > "${IMAGEPATH}" || {
  echo "Runtime Error, exiting...";
  exit 1
}

command -v kitten icat &> /dev/null || {
  echo "Seems like you aren't running on a terminal supporting the kitty graphics protocol... try opening ${IMAGEPATH} manually";
  exit 1
}
kitten icat "${IMAGEPATH}"
