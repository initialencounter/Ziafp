import os
import keyboard
from utils.doc import prepare_doc_file
from utils.docx import prepare_docx_file
from utils.utils import popup_message

signature_img_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "signature.png")
source_image_doc_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "image.doc")


def on_edit_doc():
    try:
        prepare_doc_file(source_image_doc_path)
    except Exception as e:
        popup_message("修改失败", str(e))
        
def on_edit_docx():
    try:
        prepare_docx_file(signature_img_path)
    except Exception as e:
        print("修改失败", str(e))
        
        
def main():
    keyboard.add_hotkey('ctrl+s', on_edit_docx)
    keyboard.add_hotkey('ctrl+g', on_edit_doc)
    keyboard.wait('q')  # 按下q键退出

if __name__ == "__main__":
    main()
