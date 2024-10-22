use reqwest::multipart::{Form, Part};
use std::error::Error;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
/// 目前只有这种方式能成功上传服务号素材
/// 需要有正确的content-length数值，
/// 否则无法上传成功
/// 不知道为什么multipart方法没有默认设置Content-Length
/// 没有设置Content-Length会导致412
/// 设置错误的Content-Length会返回media data missing hint错误
#[tokio::test]
async fn test() -> Result<(), Box<dyn Error>> {
    // 构造 multipart/form-data body 并返回其字节形式
    fn build_multipart_body(
        boundary: &str,
        field_name: &str,
        file_name: &str,
        file_type: &str,
        file_size: u64,
    ) -> String {
        let mut body = String::new();

        // 起始边界
        body.push_str(&format!("--{}\r\n", boundary));

        // 添加文件部分
        body.push_str(&format!(
            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
            field_name, file_name
        ));
        body.push_str(&format!("Content-Type: {}\r\n\r\n", file_type));

        // 模拟文件的内容（这里只是示例，实际应该是文件的二进制内容）
        body.push_str(&"x".repeat(file_size as usize)); // 模拟文件内容大小
        body.push_str("\r\n");

        // 添加文本部分
        body.push_str(&format!("--{}\r\n", boundary));
        body.push_str("Content-Disposition: form-data; name=\"type\"\r\n\r\n");
        body.push_str("image\r\n");

        // 结束边界
        body.push_str(&format!("--{}--\r\n", boundary));

        body
    }

    let file_name = "bc.png";
    let file_type = "image/png";
    let path = format!(
        "/workspaces/rust_base_learning/tokio_learning/src/sources/{}",
        file_name
    );
    let client = reqwest::Client::new();
    let file_path = Path::new(path.as_str());

    // 获取文件的大小
    let file_metadata = fs::metadata(file_path).await.unwrap();
    let file_size = file_metadata.len();

    // 定义边界字符串
    let boundary = "----WebKitFormBoundary123456789";

    // 手动创建一个文件部分并设置 Content-Disposition
    let file_part = Part::file(file_path)
        .await
        .unwrap()
        .file_name(file_name)
        .mime_str(file_type)
        .unwrap();

    // 手动创建文本字段部分
    let text_part = Part::text("image");

    // 构造 multipart 表单
    let form = Form::new().part("media", file_part).part("type", text_part);

    // 手动计算 Content-Length
    // 这只是一个近似的计算，实际情况下可能需要更详细的计算 multipart 头、边界等的长度
    let form_data = build_multipart_body(boundary, "media", file_name, file_type, file_size);
    let content_length = form_data.len();

    // 发送 POST 请求上传文件
    let response = client
        // .post("http://192.168.21.233:8060/test/wechat-office/upload")
        .post("https://api.weixin.qq.com/cgi-bin/material/add_material?access_token=85_J-Ip16Qu91Hwx68sA-RzXdQhubDDeuWFI4TR2X9T1J2p8ARi04RvaxgJvsORg7bFaMha9BgGmTHLR5jY8ZvbTJH6vT7QaomWpdUOAPEEZKGQdN3YZoTevwHs7jEOHBeAEAWPJ") // 替换为你的上传URL
        .header("Accept", "*/*")
        .header("Content-Length", content_length.to_string()) // 精确设置 Content-Length
        .header("Content-Type", format!("multipart/form-data; boundary={}", boundary)) // 设置边界
        .multipart(form) // 传递构建好的 multipart body
        .send()
        .await?;

    // 打印响应状态
    println!(">>>>>>>>>>>>>>>>>>{:?}", response);
    println!("Response: {:?}", response.text().await?);
    Ok(())
}

#[tokio::test]
async fn read_file() {
    use tokio::fs::File;
    let file_path = Path::new("/workspaces/rust_base_learning/tokio_learning/src/sources/test.png");
    let mut file = File::open(file_path).await.unwrap();
}

/// 没有设置
#[tokio::test]
async fn test_multipart() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let file_path = Path::new("/workspaces/rust_base_learning/tokio_learning/src/sources/test.jpg");

    // 获取文件的大小
    let file_metadata = fs::metadata(file_path).await?;
    let file_size = file_metadata.len();

    // 手动创建一个文件部分并设置 Content-Disposition
    let file_part = Part::file(file_path)
        .await
        .unwrap()
        .file_name("test.jpg")
        .mime_str("image/jpeg")?;

    // 手动创建文本字段部分
    let text_part = Part::text("image");

    // 构造 multipart 表单
    let form = Form::new().part("media", file_part).part("type", text_part);

    // 发送 POST 请求上传文件
    let response = client
        .post("https://api.weixin.qq.com/cgi-bin/material/add_material?access_token=85_r7G-1NwWLPQlzMc6CUOC7d1BhOAwcJV4qs3CFXeVnG6nd8aiIp9jOvuOOp18RM1wwF4nOWhBGiXv7dxeQU3DXXp0M8wjfB4_rrQYVmhT02Zi7vQr1i84pKt2O5IBUPfAAAFYT")
        .header("Accept", "*/*")
        .multipart(form)
        .send()
        .await?;

    // 打印响应状态
    println!("Response: {:?}", response);
    println!("Response: {:?}", response.text().await?);
    Ok(())
}

#[tokio::test]
async fn test_self_muti() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let file_path = Path::new("/workspaces/rust_base_learning/tokio_learning/src/sources/test.jpg");

    // 获取文件的内容
    let file_content = fs::read(file_path).await.unwrap();
    let file_size = file_content.len();

    // 定义边界字符串
    let boundary = "----WebKitFormBoundary123456789";

    // 手动构造 multipart/form-data body
    let mut form_data = String::new();

    // 文件部分
    form_data.push_str(&format!("--{}\r\n", boundary));
    form_data.push_str(&format!(
        "Content-Disposition: form-data; name=\"media\"; filename=\"test.jpg\"\r\n"
    ));
    form_data.push_str("Content-Type: image/jpeg\r\n\r\n");
    form_data.push_str(&String::from_utf8_lossy(&file_content));
    form_data.push_str("\r\n");

    // 文本部分
    form_data.push_str(&format!("--{}\r\n", boundary));
    form_data.push_str("Content-Disposition: form-data; name=\"type\"\r\n\r\n");
    form_data.push_str("image\r\n");

    // 结束边界
    form_data.push_str(&format!("--{}--\r\n", boundary));

    let content_length = form_data.len();

    // 发送 POST 请求上传文件
    let response = client
        .post("https://api.weixin.qq.com/cgi-bin/material/add_material?access_token=85_J-Ip16Qu91Hwx68sA-RzXdQhubDDeuWFI4TR2X9T1J2p8ARi04RvaxgJvsORg7bFaMha9BgGmTHLR5jY8ZvbTJH6vT7QaomWpdUOAPEEZKGQdN3YZoTevwHs7jEOHBeAEAWPJ")
        .header("Accept", "*/*")
        .header("Content-Length", content_length.to_string())
        .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
        .body(form_data)
        .send()
        .await?;

    // 打印响应状态
    println!("Response: {:?}", response.text().await?);
    Ok(())
}

#[tokio::test]
async fn html_file_convert() {
    let file_path =
        Path::new("/workspaces/rust_base_learning/tokio_learning/src/sources/test.html");

    // 异步读取文件内容
    let html_file_content = fs::read_to_string(file_path).await.unwrap();

    // 将字节数组转换为字符串并打印
    println!("{}", html_file_content);
}
