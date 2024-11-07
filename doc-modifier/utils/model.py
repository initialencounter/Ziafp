from pydantic import BaseModel

# 定义请求体的数据模型
class EditDocxRequest(BaseModel):
    source_path: str
    project_no: str
    date: str
    signature_img_path: str
    
class EditDocRequest(BaseModel):
    source_path: str
    save_dir: str
    project_no: str
    project_name: str
    is_965: bool
    is_power_bank: bool
    en_name: str = ''