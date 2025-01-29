cargo build --release
$tmp_dir = New-Guid
mkdir $tmp_dir
cd $tmp_dir
cp ..\target\release\rsm.exe .
cp -r ..\assets .
tar -c -a -f rsm.zip rsm.exe assets\*
cp rsm.zip ..
cd ..
rmdir -r $tmp_dir
