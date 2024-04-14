---
title: ImageProcessWebsite v1.0.0
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.17"

---

# ImageProcessWebsite

> v1.0.0

Base URLs:

# User

## POST 登录接口

POST /image_processing_website_api/login

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|username|query|string| 否 |用户名|
|password|query|string| 否 |密码|

> 返回示例

> 成功

```json
{
  "status": 0,
  "authority": 1,
  "token": "manager",
  "time_stamp": 1707546946
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，<br />1为密码错误，<br />2为权限错误，|
|» authority|integer|true|none|权限|0为未登录状态，1为普通用户，2为管理员|
|» token|string|true|none|令牌|由user_id, time_stamp, ip addr 加密所得到的哈希值|
|» time_stamp|integer|true|none|时间戳|none|
|» manager_url|object¦null|true|none||当为管理员权限时，返回manager的api_url，当为普通用户是为null|
|»» getAllWebFiles|string|true|none||none|
|»» getAllOperatingMsg|string|true|none||none|
|»» history_operations|object|true|none||none|
|»»» getAllHistoryOperation|string|true|none||none|
|»»» getOnceOfHistoryOperation|string|true|none||none|
|»» suggestion|object|true|none||none|
|»»» getAllSuggestions|string|true|none||none|
|»»» ignoreSuggestionByID|string|true|none||none|
|»»» submitResponseToSuggestionByID|string|true|none||none|
|»» user|object|true|none||none|
|»»» getAllUserMsg|string|true|none||none|
|»»» eraseUserMsg|string|true|none||none|
|»»» changeUserPwd|string|true|none||none|

## POST 注册接口

POST /image_processing_website_api/register

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|username|query|string| 否 |用户名|
|password|query|string| 否 |密码|
|time_stamp|query|integer| 否 |注册时间戳|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

## POST 管理员权限检测接口

POST /image_processing_website_api/checkManagerAuthority

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

## POST 重新设置密码接口

POST /image_processing_website_api/resetPassword

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|username|query|string| 否 |用户名|
|old_password|query|string| 否 |旧密码，如果检查到token为管理员且uer_name不是管理员身份的话可以为null|
|new_password|query|string| 否 |新密码|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

## POST 密码检查接口

POST /image_processing_website_api/checkPassword

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|username|query|string| 否 |用户名|
|password|query|string| 否 |密码|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

# About

## GET 介绍内容接口

GET /image_processing_website_api/about

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|target_content_title|query|string| 否 |介绍内容主题|

> 返回示例

> 成功

```json
{
  "status": 0,
  "introduction": "amet",
  "offical_url": [
    "http://rmydubrek.nu/dnhyjv",
    "http://bbyirjhqwp.bo/ammwyyst"
  ],
  "download_url": "http://tjvohrhb.km/wrtrjnkoo",
  "recommended_article_url": [
    "http://ibce.tt/wggf",
    "http://tlunl.kp/gwkvcf"
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|none|
|» introduction|string|true|none|介绍|none|
|» offical_url|[object]|true|none|官方链接|none|
|»» url|string|true|none||none|
|»» title|string|true|none||none|
|» download_url|[object]|true|none|下载链接|none|
|»» url|string|true|none||none|
|»» title|string|true|none||none|
|» recommended_article_url|[object]|true|none|推荐文章链接|none|
|»» url|string|true|none||none|
|»» title|string|true|none||none|

# Suggestion

## GET 提交建议接口

GET /image_processing_website_api/submit_suggestion

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|content|query|string| 否 |内容|
|time_stamp|query|integer| 否 |创建时间戳|
|token|query|string| 否 |令牌|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

# Operation/History

## GET 获取历史操作列表

GET /image_processing_website_api/history_operations

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|username|query|string| 否 |用户名|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "history_operations": [
    {
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|
|» history_operations|[object]|true|none|历史操作列表|none|
|»» time_stamp|integer|true|none|创建时间戳|none|
|»» note|string|true|none|备注|none|
|»» history_operation_id|string|true|none|历史操作id|none|

## POST 删除历史操作

POST /image_processing_website_api/earse_history_operation

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|history_operation_id|query|string| 否 |历史操作id|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|

## POST 从历史操作ID中获取操作信息接口

POST /image_processing_website_api/get_operation_details_by_history_operation_id

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|
|history_operation_id|query|string| 否 |历史操作id|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "operation_details": [
    {
      "operation_id": "string",
      "module_name": "string",
      "method_name": "string",
      "args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "output_image": [
        {
          "img_id": "string",
          "src_operation_id": "string"
        }
      ],
      "time_stamp": 0
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|0为成功，1为失败|
|» operation_details|[[Operation](#schemaoperation)]|true|none|操作详情|所有操作列表|
|»» operation_id|string|true|none||none|
|»» module_name|string|true|none||none|
|»» method_name|string|true|none||none|
|»» args|[[ArgPlaceholder](#schemaargplaceholder)]|true|none||none|
|»»» arg_id|string|true|none||none|
|»»» dst_operation_id|string|true|none||none|
|»» output_image|[[ImagePlaceholder](#schemaimageplaceholder)]|true|none||none|
|»»» img_id|string|true|none||none|
|»»» src_operation_id|string|true|none||none|
|»» time_stamp|integer|true|none||none|

# Operation/Operation/Mat

## POST read_img

POST /image_processing_website_api/operation/mat/read_img

> Body 请求参数

```yaml
token: string
file_name: string
file: string

```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|body|body|object| 否 |none|
|» token|body|string| 否 |none|
|» file_name|body|string| 否 |none|
|» file|body|string(binary)| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|
|» mat_index|integer|true|none||none|

## POST save_img

POST /image_processing_website_api/operation/mat/save_img

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat_index|query|integer| 否 |none|

> 返回示例

> 200 Response

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» file_name|string|true|none||none|
|» file|string|true|none||none|

## POST free_img

POST /image_processing_website_api/operation/mat/free_img

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat_index|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|

## POST copy

POST /image_processing_website_api/operation/mat/copy

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|src_mat_index|query|integer| 否 |none|
|dst_mat_index|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST hstack

POST /image_processing_website_api/operation/mat/hstack

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat_index_vec|query|array[string]| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST vstack

POST /image_processing_website_api/operation/mat/vstack

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat_index_vec|query|array[string]| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST resize

POST /image_processing_website_api/operation/mat/resize

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat_index|query|integer| 否 |none|
|width|query|integer| 否 |none|
|height|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

# Operation/Operation/NumericCalculation

## POST add_between_mats

POST /image_processing_website_api/operation/numeric_calculation/add_between_mats

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST add_between_mat_and_value

POST /image_processing_website_api/operation/numeric_calculation/add_between_mat_and_value

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|value|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST add_between_mat_and_scalar

POST /image_processing_website_api/operation/numeric_calculation/add_between_mat_and_scalar

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|r|query|integer| 否 |none|
|g|query|integer| 否 |none|
|b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST sub_between_mats

POST /image_processing_website_api/operation/numeric_calculation/sub_between_mats

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST sub_between_mat_and_value

POST /image_processing_website_api/operation/numeric_calculation/sub_between_mat_and_value

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|value|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST sub_between_mat_and_scalar

POST /image_processing_website_api/operation/numeric_calculation/sub_between_mat_and_scalar

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|r|query|integer| 否 |none|
|g|query|integer| 否 |none|
|b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST bitwiseAnd

POST /image_processing_website_api/operation/numeric_calculation/bitwise_and

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST bitwiseOr

POST /image_processing_website_api/operation/numeric_calculation/bitwise_or

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST bitwiseNot

POST /image_processing_website_api/operation/numeric_calculation/bitwise_not

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST bitwiseXor

POST /image_processing_website_api/operation/numeric_calculation/bitwise_xor

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|img_a|query|integer| 否 |none|
|img_b|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

# Operation/Operation/AffineTransform

## POST leftRotate90

POST /image_processing_website_api/operation/affine_transform/leftRotate90

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|tokn|query|string| 否 |none|
|mat|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST rightRotate90

POST /image_processing_website_api/operation/affine_transform/rightRotate90

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

## POST flip

POST /image_processing_website_api/operation/affine_transform/flip

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|mat|query|integer| 否 |none|
|flip_code|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "mat_index": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|[ImageIndexResponse](#schemaimageindexresponse)|

# Manager/Getters

## POST 获取所有建议内容

POST /image_processing_website_api/manager/get_all_suggestions

该接口每次调用，都会返回二十条新的建议信息，直到无为止

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 是 |令牌|
|now_len|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "suggestions": [
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    },
    {
      "username": "string",
      "content": "string",
      "suggestion_id": 0,
      "time_stamp": 0
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none|状态码|none|
|» suggestions|[object]|true|none|建立列表|none|
|»» username|string|true|none||none|
|»» content|string|true|none||none|
|»» suggestion_id|integer|true|none||none|
|»» time_stamp|integer|true|none||none|

## POST 获取所有用户信息

POST /image_processing_website_api/manager/get_all_user_msg

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "user_msg": [
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    },
    {
      "username": "string",
      "authority": 0,
      "time_stamp": 0,
      "history_operation_count": 0,
      "result_image_count": 0
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|
|» user_msg|[object]|true|none||none|
|»» username|string|true|none||none|
|»» authority|integer|true|none||none|
|»» time_stamp|integer|true|none||none|
|»» history_operation_count|integer|true|none||none|
|»» result_image_count|integer|true|none||none|

## POST 获取所有历史操作

POST /image_processing_website_api/manager/get_all_history_operation

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0,
  "history_operations": [
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    },
    {
      "init_args": [
        {
          "arg_id": "string",
          "dst_operation_id": "string"
        }
      ],
      "operations": [],
      "time_stamp": 0,
      "note": "string",
      "history_operation_id": "string"
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|
|» history_operations|[[HistoryOperation](#schemahistoryoperation)]|true|none||none|
|»» init_args|[[ArgPlaceholder](#schemaargplaceholder)]|true|none||none|
|»»» arg_id|string|true|none||none|
|»»» dst_operation_id|string|true|none||none|
|»» operations|[[Operation](#schemaoperation)]¦null|true|none||none|
|»»» operation_id|string|true|none||none|
|»»» module_name|string|true|none||none|
|»»» method_name|string|true|none||none|
|»»» args|[[ArgPlaceholder](#schemaargplaceholder)]|true|none||none|
|»»»» arg_id|string|true|none||none|
|»»»» dst_operation_id|string|true|none||none|
|»»» output_image|[[ImagePlaceholder](#schemaimageplaceholder)]|true|none||none|
|»»»» img_id|string|true|none||none|
|»»»» src_operation_id|string|true|none||none|
|»»» time_stamp|integer|true|none||none|
|»» time_stamp|integer|true|none||none|
|»» note|string|true|none||none|
|»» history_operation_id|string|true|none||none|

## POST 获取网页文件

POST /image_processing_website_api/manager/get_all_webfiles

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## POST 获取所有操作信息

POST /image_processing_website_api/manager/get_all_operating_msg

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |令牌|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## POST 获取单个历史操作

POST /image_processing_website_api/manager/get_once_of_history_operation

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

# Manager/Commitment

## POST 提交意见反馈

POST /image_processing_website_api/manager/submit_response_to_suggestion_by_id

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|suggestion_id|query|string| 否 |建议id|
|response|query|string| 否 |反馈|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|

## POST 删除建议

POST /image_processing_website_api/manager/ignore_suggestion_by_id

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|suggestion_id|query|string| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|

## POST 删除用户

POST /image_processing_website_api/manager/erase_user_msg

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|token|query|string| 否 |none|
|username|query|string| 否 |none|

> 返回示例

> 200 Response

```json
{
  "status": 0
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» status|integer|true|none||none|

# 数据模型

<h2 id="tocS_ImageIndexResponse">ImageIndexResponse</h2>

<a id="schemaimageindexresponse"></a>
<a id="schema_ImageIndexResponse"></a>
<a id="tocSimageindexresponse"></a>
<a id="tocsimageindexresponse"></a>

```json
{
  "status": 0,
  "mat_index": 0
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|status|integer|true|none||none|
|mat_index|integer|true|none||none|

<h2 id="tocS_HistoryOperation">HistoryOperation</h2>

<a id="schemahistoryoperation"></a>
<a id="schema_HistoryOperation"></a>
<a id="tocShistoryoperation"></a>
<a id="tocshistoryoperation"></a>

```json
{
  "init_args": [
    {
      "arg_id": "string",
      "dst_operation_id": "string"
    }
  ],
  "operations": [],
  "time_stamp": 0,
  "note": "string",
  "history_operation_id": "string"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|init_args|[[ArgPlaceholder](#schemaargplaceholder)]|true|none||none|
|operations|[[Operation](#schemaoperation)]¦null|true|none||none|
|time_stamp|integer|true|none||none|
|note|string|true|none||none|
|history_operation_id|string|true|none||none|

<h2 id="tocS_ImagePlaceholder">ImagePlaceholder</h2>

<a id="schemaimageplaceholder"></a>
<a id="schema_ImagePlaceholder"></a>
<a id="tocSimageplaceholder"></a>
<a id="tocsimageplaceholder"></a>

```json
{
  "img_id": "string",
  "src_operation_id": "string"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|img_id|string|true|none||none|
|src_operation_id|string|true|none||none|

<h2 id="tocS_ArgPlaceholder">ArgPlaceholder</h2>

<a id="schemaargplaceholder"></a>
<a id="schema_ArgPlaceholder"></a>
<a id="tocSargplaceholder"></a>
<a id="tocsargplaceholder"></a>

```json
{
  "arg_id": "string",
  "dst_operation_id": "string"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|arg_id|string|true|none||none|
|dst_operation_id|string|true|none||none|

<h2 id="tocS_Operation">Operation</h2>

<a id="schemaoperation"></a>
<a id="schema_Operation"></a>
<a id="tocSoperation"></a>
<a id="tocsoperation"></a>

```json
{
  "operation_id": "string",
  "module_name": "string",
  "method_name": "string",
  "args": [
    {
      "arg_id": "string",
      "dst_operation_id": "string"
    }
  ],
  "output_image": [
    {
      "img_id": "string",
      "src_operation_id": "string"
    }
  ],
  "time_stamp": 0
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|operation_id|string|true|none||none|
|module_name|string|true|none||none|
|method_name|string|true|none||none|
|args|[[ArgPlaceholder](#schemaargplaceholder)]|true|none||none|
|output_image|[[ImagePlaceholder](#schemaimageplaceholder)]|true|none||none|
|time_stamp|integer|true|none||none|

