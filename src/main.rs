#![allow(non_snake_case)]
#![allow(deprecated)]

use serde_json::json;
use qrcode::QrCode;
use image::{Rgb, RgbImage, Luma, DynamicImage};
use imageproc::drawing::{draw_text_mut, text_size};
use base64::encode;
use std::io::{self, Write};
use std::env;
use chrono::{NaiveDate, TimeZone};
use ab_glyph::{FontRef, PxScale};

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let cardNo;
    let year;
    let month;
    let day;

    if args.len() == 5 {
        cardNo = args[1].clone();
        year = args[2].parse().expect("Please enter a valid year");
        month = args[3].parse().expect("Please enter a valid month");
        day = args[4].parse().expect("Please enter a valid day");
    } else {
        // 获取用户输入的卡号
        print!("请输入卡号：");
        handle.flush().unwrap();
        let mut cardNo_input = String::new();
        io::stdin().read_line(&mut cardNo_input).expect("Failed to read line");
        cardNo = cardNo_input.trim().to_string();

        // 获取用户输入的年份
        print!("请输入年份：");
        handle.flush().unwrap();
        let mut year_str = String::new();
        io::stdin().read_line(&mut year_str).expect("Failed to read line");
        year = year_str.trim().parse().expect("Please enter a valid year");

        // 获取用户输入的月份
        print!("请输入月份：");
        handle.flush().unwrap();
        let mut month_str = String::new();
        io::stdin().read_line(&mut month_str).expect("Failed to read line");
        month = month_str.trim().parse().expect("Please enter a valid month");

        // 获取用户输入的日期
        print!("请输入日期：");
        handle.flush().unwrap();
        let mut day_str = String::new();
        io::stdin().read_line(&mut day_str).expect("Failed to read line");
        day = day_str.trim().parse().expect("Please enter a valid day");
    }

    // 在获取日期之前，将时区设置为东八区（北京时间）
    let beijing_time = chrono::offset::FixedOffset::east(8 * 3600);
    
    // 构建日期对象
    let naive_date_utc = beijing_time.from_utc_date(&NaiveDate::from_ymd(year, month, day));

    // 获取当天凌晨的时间戳
    let timeStamp = naive_date_utc.and_hms(0, 0, 0).timestamp();

    // 构建 JSON 数据
    let data = json!({
        "cardNo": cardNo,
        "timeStamp": timeStamp
    });

    // 将 JSON 数据转换为字符串
    let json_string = serde_json::to_string(&data).unwrap();

    // 将 JSON 字符串编码为 Base64
    let base64_encoded = encode(&json_string);
    println!("base64: {}", base64_encoded);

    // 创建二维码
    let code = QrCode::new(base64_encoded).unwrap();

    // 渲染二维码图像到指定大小
    let qr_image = code.render::<Luma<u8>>()
        .max_dimensions(160, 160) // 指定图像大小
        .min_dimensions(160, 160) // 指定最小图像大小
        .module_dimensions(2, 2) // 设置模块大小
        .quiet_zone(false) // 关闭静默区
        .build();

    // 将二维码图像转换为 RGB
    let qr_rgb_image: RgbImage = DynamicImage::ImageLuma8(qr_image).to_rgb8();

    // 构建文件名
    let naive_date = NaiveDate::from_ymd(year, month, day);
    let filename = format!("{}_{}.png", cardNo, naive_date);

    // 生成带文字的图像
    let font_data = include_bytes!("../msyh.ttc");
    let font = FontRef::try_from_slice(font_data).unwrap();
    let scale = PxScale::from(20.0);

    // 文本内容
    let date_time = format!("有效日期：{}年{}月{}日", year, month, day);

    // 计算文本大小
    let (text_time_width, text_time_height) = text_size(scale, &font, &date_time);

    // 设置背景尺寸
    let bg_width = 200;
    let bg_height = qr_rgb_image.height() + text_time_height as u32 + 120;

    // 创建白色背景的图像
    let mut image = RgbImage::from_pixel(bg_width, bg_height, Rgb([255, 255, 255]));

    // 计算二维码和文字的中心位置
    let qr_x = (bg_width - qr_rgb_image.width()) / 2;
    let text_time_x = (bg_width - text_time_width as u32) / 2;
    let text_time_y = qr_rgb_image.height() + 80;

    // 将二维码图像叠加到白色背景上
    image::imageops::overlay(&mut image, &qr_rgb_image, qr_x.into(), 60);

    // 添加文本
    draw_text_mut(&mut image, Rgb([0, 0, 0]), text_time_x as i32, text_time_y as i32, scale, &font, &date_time);

    // let date_name = format!("");
    // let (date_name_width, _date_name_height) = text_size(scale, &font, &date_name);
    // let text_name_x = (bg_width - date_name_width as u32) / 2;
    // let text_name_y = 10;
    // draw_text_mut(&mut image, Rgb([0, 0, 0]), text_name_x as i32, text_name_y, scale, &font, &date_name);

    // 保存二维码图片
    image.save(&filename).unwrap();

    println!("二维码已保存为 {}", filename);
}
