from fastapi import FastAPI, HTTPException
from utils.doc import edit_doc_file
from utils.docx import edit_docx_file 
from utils.model import EditDocRequest, EditDocxRequest

app = FastAPI()


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

@app.post("/edit-docx")
async def edit_docx(request: EditDocxRequest):
    print(request)
    try:
        edit_docx_file(request.source_path, request.signature_img_path)
        return {"message": "Document edited successfully"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
    

# 启动应用程序
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, port=25457)
# uvicorn main:app --port 25457
# .\venv\Scripts\uvicorn.exe main:app --port 25457

## packed
# pyinstaller --name doc_modifier --add-data "utils;utils" --hidden-import=uvicorn.logging --hidden-import=uvicorn.loops --hidden-import=uvicorn.loops.auto --hidden-import=uvicorn.protocols --hidden-import=uvicorn.protocols.http --hidden-import=uvicorn.protocols.http.auto --hidden-import=uvicorn.protocols.websockets --hidden-import=uvicorn.protocols.websockets.auto --hidden-import=uvicorn.lifespan --hidden-import=uvicorn.lifespan.on --hidden-import=uvicorn.lifespan.off --onefile run.py