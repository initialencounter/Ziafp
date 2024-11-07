import requests

def get_project_info(project_no: str):
    url = f"http://localhost:25455/get-project-info/{project_no}"
    
    # 发送GET请求
    response = requests.get(url)
    
    # 检查响应状态
    if response.status_code == 200:
        res = response.json()  # 假设QueryResult可以直接从JSON反序列化
        return res
    else:
        raise Exception("未找到项目信息")
