import os
import win32com.client
import pythoncom
from utils.explorer import get_explorer_path
from utils.utils import open_file_with_default_program,  popup_message, check_project_no, get_clip_text
from utils.api import get_project_info

def edit_doc_file(source_path, save_dir, project_no, project_name, is_965, is_power_bank, en_name=''):
    try:
        # 创建 Word 应用程序对象
        # 如果是wps 则改成KWPS.Application 如果是office 则改成word.Application.16
        word = win32com.client.Dispatch("KWPS.Application")
        word.Visible = False

        # 打开文档
        doc = word.Documents.Open(source_path)
        range = doc.Content
        
        if is_965:
            if is_power_bank:
                range.Paragraphs(1).Range.Delete()
                range.InsertBefore(f"{project_name.split(' ')[0]}{en_name}\n")
            # 删除设备
            third_paragraph = range.Paragraphs(3)
            third_paragraph.Range.Delete()
            fourth_paragraph = range.Paragraphs(3)
            fourth_paragraph.Range.Delete()

        range.InsertBefore(f"物品名称：{project_name}\n")
        range.InsertBefore(f"项目编号：{project_no}\n")
        
        # 设置第一行的行距为1.0
        first_paragraph = range.Paragraphs(1)
        first_paragraph.Format.LineSpacingRule = 0
        # 如果是wps 则改成word.LinesToPoints(1) 如果是office 则改成12
        first_paragraph.Format.LineSpacing = word.LinesToPoints(1.0) 
        first_paragraph.Alignment = 0

        # 设置第二行的行距为1.5
        second_paragraph = range.Paragraphs(2)
        second_paragraph.Format.LineSpacingRule = 0
        second_paragraph.Format.LineSpacing = word.LinesToPoints(1.5)
        second_paragraph.Alignment = 0
        
        # 设置第三行的行距为1.5
        third_paragraph = range.Paragraphs(3)
        third_paragraph.Format.LineSpacingRule = 0
        third_paragraph.Format.LineSpacing = word.LinesToPoints(1.5)
        third_paragraph.Alignment = 0
        save_path = os.path.join(save_dir, f"{project_no}.doc")
        
        # 保存文档
        doc.SaveAs(save_path)
        doc.Close()
        doc = None
        return save_path
    except pythoncom.com_error as e:
        print("COM error occurred:", e)
    finally:
        # 仅在没有打开文档时退出 Word
        if 'word' in locals() and hasattr(word, 'Quit'):
            word.Quit()
            word = None

def prepare_doc_file(source_image_doc_path):
    path = get_explorer_path()
    if not path:
        return
    project_no = get_clip_text()
    if not check_project_no(project_no):
        return
    project_info = get_project_info(project_no)["rows"][0]
    if not project_info:
        popup_message("项目信息获取失败", "项目信息获取失败")
        return
    if not check_project_no(project_no):
        popup_message("项目号格式不正确", "项目号格式不正确")
        return
    
    en_name = project_info["itemEName"].split(" ")[0]
    is_965 = "内置" in project_info["itemCName"] or "包装" in project_info["itemCName"]
    is_power_bank = "移动电源" in project_info["itemCName"] or "储能电源" in project_info["itemCName"]
    confirm = popup_message("确认", f"项目名称：{project_info['itemCName']}\n项目编号：{project_no}\n项目英文名称：{en_name}\n是否965：{is_965}\n是否移动电源：{is_power_bank}")
    if not confirm:
        print("用户取消")
        return
    save_path = edit_doc_file(source_path=source_image_doc_path,
                    save_dir=path, 
                    project_no=project_no, 
                    project_name=project_info["itemCName"],
                    is_965=is_965,
                    is_power_bank=is_power_bank,
                    en_name=en_name)
    open_file_with_default_program(save_path)