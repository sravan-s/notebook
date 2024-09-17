rm -rf ./assets
mkdir assets

rm -rf ./agent
mkdir agent

./build_agent.sh

./download_vmlinux.sh
./make_rootfs.sh

