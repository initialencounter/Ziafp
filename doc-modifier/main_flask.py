from flask import Flask, request, jsonify
from utils.doc import edit_doc_file
from utils.docx import edit_docx_file 

app = Flask(__name__)

@app.route("/edit-doc", methods=['POST'])
def edit_doc():
    try:
        data = request.get_json()
        save_path = edit_doc_file(
            source_path=data['source_path'],
            save_dir=data['save_dir'],
            project_no=data['project_no'],
            project_name=data['project_name'],
            is_965=data['is_965'],
            is_power_bank=data['is_power_bank'],
            en_name=data['en_name']
        )
        return jsonify({"message": "Document edited successfully", "save_path": save_path})
    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route("/edit-docx", methods=['POST'])
def edit_docx():
    data = request.get_json()
    print(data)
    try:
        edit_docx_file(data['source_path'], data['signature_img_path'])
        return jsonify({"message": "Document edited successfully"})
    except Exception as e:
        return jsonify({"error": str(e)}), 500
      
if __name__ == "__main__":
    app.run(port=25457)