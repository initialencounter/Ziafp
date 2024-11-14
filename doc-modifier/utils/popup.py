from utils.api import get_project_info
from utils.docx import edit_docx_file, replace_last_image_in_docx
from utils.explorer import get_explorer_path
from utils.utils import check_project_no, get_clip_text, match_docx_files, open_file_with_default_program, popup_message

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


def prepare_docx_file(signature_img_path):
    path = get_explorer_path()
    if not path:
        return
    file_path_list, file_name_list = match_docx_files(path)
    if not file_path_list:
        popup_message("未找到概要文件", "未找到概要文件")
        return
    confirm = popup_message("确认", "是否要修改这些概要？\n" + "\n".join(file_name_list))
    if not confirm:
        print("用户取消")
        return
    for file_path in file_path_list:
        edit_docx_file(file_path, signature_img_path)
        open_file_with_default_program(file_path)
        replace_last_image_in_docx(file_path, signature_img_path)
