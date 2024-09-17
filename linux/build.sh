rm -rf ./assets
mkdir assets

rm -rf ./agent
mkdir agent

rm -rf ./executer

./build_agent.sh

./download_vmlinux.sh
./make_rootfs.sh

