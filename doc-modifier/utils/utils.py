import os
import re
import datetime
import ctypes
import subprocess
import sys
import pyperclip

PROJECT_NO_REGEX = re.compile(r"[P|S]EK.{2}\d{12}")

def check_project_no(project_no):
    return PROJECT_NO_REGEX.match(project_no) is not None

def get_clip_text():
    return pyperclip.paste()

def popup_message(title, message):
    # 使用Windows API显示消息框
    return ctypes.windll.user32.MessageBoxW(0, message, title, 1) == 1

def open_file_with_default_program(path):
    subprocess.run(["cmd", "/C", "start", "", path], check=True)

def match_docx_files(dir):
    file_path_list = []
    file_name_list = []
    for root, _, files in os.walk(dir):
        for file_name in files:
            if "概要" not in file_name:
                continue
            # 检查文件名是否符合要求
            if not file_name.endswith(".docx"):
                continue
            if not file_name.startswith("PEK") and not file_name.startswith("SEK"):
                continue
            file_path = os.path.join(root, file_name)
            file_path_list.append(file_path)
            file_name_list.append(file_name)
    return file_path_list , file_name_list

def get_today_date():
    # 获取当前日期
    today = datetime.date.today()

    # 格式化为 YYYY-MM-DD
    formatted_date = today.strftime("%Y-%m-%d")
    return formatted_date

def get_executable_dir():
    if getattr(sys, 'frozen', False):
        # 如果是打包后的可执行文件
        return os.path.dirname(sys.executable)
    else:
        # 如果是开发环境
        return os.path.dirname(os.path.abspath(__file__))