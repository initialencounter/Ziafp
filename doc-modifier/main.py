from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import os
import win32com.client
import pythoncom

app = FastAPI()

# 定义请求体的数据模型
class EditDocRequest(BaseModel):
    source_path: str
    save_dir: str
    project_no: str
    project_name: str
    is_965: bool
    is_power_bank: bool
    en_name: str = ''

def edit_doc_file(source_path, save_dir, project_no, project_name, is_965, is_power_bank, en_name=''):
    try:
        # 创建 Word 应用程序对象
        # 如果是wps 则改成word.Application 如果是office 则改成word.Application.16
        word = win32com.client.Dispatch("Word.Application") 
        word.Visible = False

        # 打开文档
        doc = word.Documents.Open(source_path)
        print(source_path)
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

@app.post("/edit-doc")
async def edit_doc(request: EditDocRequest):
    print(request)
    try:
        save_path = edit_doc_file(
            source_path=request.source_path,
            save_dir=request.save_dir,
            project_no=request.project_no,
            project_name=request.project_name,
            is_965=request.is_965,
            is_power_bank=request.is_power_bank,
            en_name=request.en_name
        )
        return {"message": "Document edited successfully", "save_path": save_path}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# 启动应用程序
# uvicorn main:app --port 25457
