import json
import sqlite3
from BaseObject import BaseObject
from OpenCV import OpenCV
from Rocket import Rocket
from Vue import Vue

def getSQLParamsTuple(baseobject: BaseObject):
    return (baseobject.title, baseobject.introduction, json.dumps(baseobject.offical_url), json.dumps(baseobject.dowload_url), json.dumps(baseobject.recommended_article_url))

db = sqlite3.connect("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/ImagProcessingWebsite.db")

data = [getSQLParamsTuple(OpenCV()),
        getSQLParamsTuple(Rocket()),
        getSQLParamsTuple(Vue())]

for params in data:
    try:
        db.execute("INSERT INTO ABOUT(TITLE, INTRODUCTION, OFFICAL_URL, DOWLOAD_URL, RECOMMENDED_ARTICLE_URL) VALUES (?, ?, ?, ?, ?)",
                     params)
    except:
        tmp_params = list(params)
        tmp_params.append(tmp_params[0])
        params = tuple(tmp_params)
        db.execute("UPDATE ABOUT SET TITLE=?, INTRODUCTION=?, OFFICAL_URL=?, DOWLOAD_URL=?, RECOMMENDED_ARTICLE_URL=? WHERE TITLE=?", params)

db.commit()
db.close()