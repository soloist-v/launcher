### 一个简单的启动器
Resolve Windows DLL shell issues.   
Customizable DLL folder  
cfg:
```yaml
executable: "test.exe"  # target exe file
args: [ ]  # the target exe start args
lib_dir: "lib"  # dll directory path
output: "out.log"  # exe output file, if "" will use stdout
keep_alive: False  # Does it survive after starting the exe process
```