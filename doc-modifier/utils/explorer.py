import win32com.client
import pythoncom
import win32gui

def get_explorer_path():
    # 初始化COM库
    pythoncom.CoInitialize()
    
    # 获取当前聚焦窗口的句柄
    foreground_hwnd = win32gui.GetForegroundWindow()
    
    # 创建Shell.Application对象
    shell = win32com.client.Dispatch("Shell.Application")
    
    # 遍历所有打开的窗口
    for window in shell.Windows():
        # 检查窗口是否是资源管理器窗口并且是当前聚焦的窗口
        if (window.Name == "文件资源管理器" or window.Name == "Windows 资源管理器") and window.HWND == foreground_hwnd:
            # 返回当前路径
            return window.LocationURL.replace('file:///', '')
    
    return None

def on_hotkey():
    # 获取当前聚焦的资源管理器路径
    path = get_explorer_path()
    if path:
        print("当前路径:", path)
    else:
        print("未找到资源管理器窗口")

if __name__ == "__main__":
    import keyboard
    # 监听快捷键 Ctrl+Shift+E
    keyboard.add_hotkey('ctrl+shift+e', on_hotkey)

    # 保持程序运行
    keyboard.wait('esc')  # 按下Esc键退出
    print(get_explorer_path())

