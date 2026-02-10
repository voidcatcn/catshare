//! Internationalization translations module
//! Supports 10 common languages: English, Chinese, Spanish, Hindi, Arabic, Portuguese, Russian, Japanese, German, French

use std::collections::HashMap;
use std::sync::OnceLock;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    English,
    Chinese,    // 简体中文
    Spanish,    // Español
    Hindi,      // हिन्दी
    Arabic,     // العربية
    Portuguese, // Português
    Russian,    // Русский
    Japanese,   // 日本語
    German,     // Deutsch
    French,     // Français
}

impl Language {
    pub fn from_system() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        let code = locale
            .split(|c: char| !c.is_alphabetic())
            .next()
            .unwrap_or("en")
            .to_lowercase();

        match code.as_str() {
            "zh" => Language::Chinese,
            "es" => Language::Spanish,
            "hi" => Language::Hindi,
            "ar" => Language::Arabic,
            "pt" => Language::Portuguese,
            "ru" => Language::Russian,
            "ja" => Language::Japanese,
            "de" => Language::German,
            "fr" => Language::French,
            _ => Language::English,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Spanish => "es",
            Language::Hindi => "hi",
            Language::Arabic => "ar",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Japanese => "ja",
            Language::German => "de",
            Language::French => "fr",
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "简体中文",
            Language::Spanish => "Español",
            Language::Hindi => "हिन्दी",
            Language::Arabic => "العربية",
            Language::Portuguese => "Português",
            Language::Russian => "Русский",
            Language::Japanese => "日本語",
            Language::German => "Deutsch",
            Language::French => "Français",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇺🇸",
            Language::Chinese => "🇨🇳",
            Language::Spanish => "🇪🇸",
            Language::Hindi => "🇮🇳",
            Language::Arabic => "🇦🇪",
            Language::Portuguese => "🇵🇹",
            Language::Russian => "🇷🇺",
            Language::Japanese => "🇯🇵",
            Language::German => "🇩🇪",
            Language::French => "🇫🇷",
        }
    }

    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Chinese,
            Language::Spanish,
            Language::Hindi,
            Language::Arabic,
            Language::Portuguese,
            Language::Russian,
            Language::Japanese,
            Language::German,
            Language::French,
        ]
    }
}

/// Translation keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // App
    AppTitle,
    AppSubtitle,

    // Identity
    YourName,
    YourId,
    WaitingForConnection,

    // Send
    Send,
    SendFile,
    SendFiles,
    SelectFiles,
    SelectedFiles,
    EnterId,
    EnterReceiverIdHint,
    WaitingConfirmation,
    SendingTo,
    Compress,
    StartSend,

    // Receive
    Receive,
    ReceiveFile,
    ReceiveFiles,
    WaitingForSender,
    IncomingRequest,
    Accept,
    Reject,
    ReceivingFrom,
    SaveTo,
    SelectFolder,

    // Progress
    Progress,
    Compressing,
    Transferring,
    Decompressing,
    Complete,
    Failed,
    Cancelled,
    Speed,
    TimeRemaining,
    BytesTransferred,

    // Status
    Connected,
    Disconnected,
    Connecting,
    Error,

    // Settings
    Settings,
    Language,
    Theme,
    DarkMode,
    LightMode,
    CompressionLevel,
    ThreadCount,
    BufferSize,

    // Buttons
    Cancel,
    Confirm,
    Retry,
    Close,
    Browse,
    Clear,
    CopyId,
    Refresh,
    EditName,
    SaveName,

    // Messages
    ConnectionFailed,
    TransferComplete,
    TransferFailed,
    InvalidId,
    NoFilesSelected,
    RequestSent,
    RequestReceived,
    RequestAccepted,
    RequestRejected,
    FileNotFound,
    PermissionDenied,
    NetworkError,

    // Tooltips
    IdTooltip,
    CompressTooltip,
    ThreadTooltip,

    // File info
    FileName,
    FileSize,
    FileCount,
    TotalSize,
    AddMore,
    StartReceive,
    Sender,
    Transferred,
    EnableCompression,
    SelectFilesHint,
    EnableEncryption,
    EnableAnimations,
    ColorTheme,
    SaveSettings,
    ElectricIndigoTheme,
    ClassicBlueTheme,
    DarkTheme,
    LightTheme,
    OrangeCatTheme,
}

/// Global translations storage
static TRANSLATIONS: OnceLock<Translations> = OnceLock::new();

/// Translations container
pub struct Translations {
    data: HashMap<Language, HashMap<TranslationKey, &'static str>>,
}

impl Translations {
    fn new() -> Self {
        let mut data = HashMap::new();

        // English
        let mut en = HashMap::new();
        en.insert(TranslationKey::AppTitle, "CatShare");
        en.insert(TranslationKey::AppSubtitle, "Fast & Easy File Sharing");
        en.insert(TranslationKey::YourName, "Your Name");
        en.insert(TranslationKey::YourId, "Your ID");
        en.insert(
            TranslationKey::WaitingForConnection,
            "Waiting for connection...",
        );
        en.insert(TranslationKey::Send, "Send");
        en.insert(TranslationKey::SendFile, "Send File");
        en.insert(TranslationKey::SendFiles, "Send Files");
        en.insert(TranslationKey::SelectFiles, "Select Files");
        en.insert(TranslationKey::SelectedFiles, "Selected Files");
        en.insert(TranslationKey::EnterId, "Enter ID");
        en.insert(TranslationKey::EnterReceiverIdHint, "Enter receiver's ID");
        en.insert(
            TranslationKey::WaitingConfirmation,
            "Waiting for confirmation...",
        );
        en.insert(TranslationKey::SendingTo, "Sending to");
        en.insert(TranslationKey::Compress, "Compress");
        en.insert(TranslationKey::StartSend, "Start Send");
        en.insert(TranslationKey::AddMore, "Add More");
        en.insert(TranslationKey::Receive, "Receive");
        en.insert(TranslationKey::ReceiveFile, "Receive File");
        en.insert(TranslationKey::ReceiveFiles, "Receive Files");
        en.insert(TranslationKey::WaitingForSender, "Waiting for sender...");
        en.insert(TranslationKey::IncomingRequest, "Incoming request");
        en.insert(TranslationKey::Accept, "Accept");
        en.insert(TranslationKey::Reject, "Reject");
        en.insert(TranslationKey::ReceivingFrom, "Receiving from");
        en.insert(TranslationKey::SaveTo, "Save to");
        en.insert(TranslationKey::SelectFolder, "Select Folder");
        en.insert(TranslationKey::Progress, "Progress");
        en.insert(TranslationKey::Compressing, "Compressing...");
        en.insert(TranslationKey::Transferring, "Transferring...");
        en.insert(TranslationKey::Decompressing, "Decompressing...");
        en.insert(TranslationKey::Complete, "Complete");
        en.insert(TranslationKey::Failed, "Failed");
        en.insert(TranslationKey::Cancelled, "Cancelled");
        en.insert(TranslationKey::Speed, "Speed");
        en.insert(TranslationKey::TimeRemaining, "Time remaining");
        en.insert(TranslationKey::BytesTransferred, "Bytes transferred");
        en.insert(TranslationKey::Transferred, "Transferred");
        en.insert(TranslationKey::Connected, "Connected");
        en.insert(TranslationKey::Disconnected, "Disconnected");
        en.insert(TranslationKey::Connecting, "Connecting...");
        en.insert(TranslationKey::Error, "Error");
        en.insert(TranslationKey::Settings, "Settings");
        en.insert(TranslationKey::Language, "Language");
        en.insert(TranslationKey::Theme, "Theme");
        en.insert(TranslationKey::DarkMode, "Dark Mode");
        en.insert(TranslationKey::LightMode, "Light Mode");
        en.insert(TranslationKey::CompressionLevel, "Compression Level");
        en.insert(TranslationKey::ThreadCount, "Thread Count");
        en.insert(TranslationKey::BufferSize, "Buffer Size");
        en.insert(TranslationKey::EnableCompression, "Enable LZ4 Compression");
        en.insert(TranslationKey::Cancel, "Cancel");
        en.insert(TranslationKey::Confirm, "Confirm");
        en.insert(TranslationKey::Retry, "Retry");
        en.insert(TranslationKey::Close, "Close");
        en.insert(TranslationKey::Browse, "Browse");
        en.insert(TranslationKey::Clear, "Clear");
        en.insert(TranslationKey::CopyId, "Copy ID");
        en.insert(TranslationKey::Refresh, "Refresh");
        en.insert(TranslationKey::EditName, "Edit Name");
        en.insert(TranslationKey::SaveName, "Save");
        en.insert(TranslationKey::ConnectionFailed, "Connection failed");
        en.insert(TranslationKey::TransferComplete, "Transfer complete!");
        en.insert(TranslationKey::TransferFailed, "Transfer failed");
        en.insert(TranslationKey::InvalidId, "Invalid ID");
        en.insert(TranslationKey::NoFilesSelected, "No files selected");
        en.insert(TranslationKey::RequestSent, "Request sent");
        en.insert(TranslationKey::RequestReceived, "Request received");
        en.insert(TranslationKey::RequestAccepted, "Request accepted");
        en.insert(TranslationKey::RequestRejected, "Request rejected");
        en.insert(TranslationKey::FileNotFound, "File not found");
        en.insert(TranslationKey::PermissionDenied, "Permission denied");
        en.insert(TranslationKey::NetworkError, "Network error");
        en.insert(TranslationKey::Sender, "Sender");
        en.insert(TranslationKey::StartReceive, "Start Receiving");
        en.insert(TranslationKey::IdTooltip, "Share this ID with the sender");
        en.insert(
            TranslationKey::CompressTooltip,
            "Enable compression for faster transfer",
        );
        en.insert(
            TranslationKey::ThreadTooltip,
            "More threads = faster transfer",
        );
        en.insert(TranslationKey::FileName, "File Name");
        en.insert(TranslationKey::FileSize, "File Size");
        en.insert(TranslationKey::FileCount, "File Count");
        en.insert(TranslationKey::TotalSize, "Total Size");
        en.insert(
            TranslationKey::SelectFilesHint,
            "or drag and drop files here",
        );
        en.insert(TranslationKey::EnableEncryption, "Enable Transfer Encryption");
        en.insert(TranslationKey::EnableAnimations, "Enable Animations");
        en.insert(TranslationKey::ColorTheme, "Color Theme");
        en.insert(TranslationKey::SaveSettings, "Save Settings");
        en.insert(TranslationKey::ElectricIndigoTheme, "Electric Indigo");
        en.insert(TranslationKey::ClassicBlueTheme, "Classic Blue");
        en.insert(TranslationKey::DarkTheme, "Dark Theme");
        en.insert(TranslationKey::LightTheme, "Light Theme");
        en.insert(TranslationKey::OrangeCatTheme, "Orange Cat Theme");
        data.insert(Language::English, en);

        // Chinese (简体中文)
        let mut zh = HashMap::new();
        zh.insert(TranslationKey::AppTitle, "猫咪分享");
        zh.insert(TranslationKey::AppSubtitle, "快速便捷的文件分享");
        zh.insert(TranslationKey::YourName, "你的名称");
        zh.insert(TranslationKey::YourId, "你的ID");
        zh.insert(TranslationKey::WaitingForConnection, "等待连接...");
        zh.insert(TranslationKey::Send, "发送");
        zh.insert(TranslationKey::SendFile, "发送文件");
        zh.insert(TranslationKey::SendFiles, "发送文件");
        zh.insert(TranslationKey::SelectFiles, "选择文件");
        zh.insert(TranslationKey::SelectedFiles, "已选文件");
        zh.insert(TranslationKey::EnterId, "输入ID");
        zh.insert(TranslationKey::EnterReceiverIdHint, "输入接收方ID");
        zh.insert(TranslationKey::WaitingConfirmation, "等待确认...");
        zh.insert(TranslationKey::SendingTo, "发送至");
        zh.insert(TranslationKey::Compress, "压缩");
        zh.insert(TranslationKey::StartSend, "开始发送");
        zh.insert(TranslationKey::AddMore, "添加更多");
        zh.insert(TranslationKey::Receive, "接收");
        zh.insert(TranslationKey::ReceiveFile, "接收文件");
        zh.insert(TranslationKey::ReceiveFiles, "接收文件");
        zh.insert(TranslationKey::WaitingForSender, "等待发送方...");
        zh.insert(TranslationKey::IncomingRequest, "收到请求");
        zh.insert(TranslationKey::Accept, "接受");
        zh.insert(TranslationKey::Reject, "拒绝");
        zh.insert(TranslationKey::ReceivingFrom, "接收自");
        zh.insert(TranslationKey::SaveTo, "保存至");
        zh.insert(TranslationKey::SelectFolder, "选择文件夹");
        zh.insert(TranslationKey::Progress, "进度");
        zh.insert(TranslationKey::Compressing, "压缩中...");
        zh.insert(TranslationKey::Transferring, "传输中...");
        zh.insert(TranslationKey::Decompressing, "解压中...");
        zh.insert(TranslationKey::Complete, "完成");
        zh.insert(TranslationKey::Failed, "失败");
        zh.insert(TranslationKey::Cancelled, "已取消");
        zh.insert(TranslationKey::Speed, "速度");
        zh.insert(TranslationKey::TimeRemaining, "剩余时间");
        zh.insert(TranslationKey::BytesTransferred, "已传输");
        zh.insert(TranslationKey::Transferred, "已传输量");
        zh.insert(TranslationKey::Connected, "已连接");
        zh.insert(TranslationKey::Disconnected, "已断开");
        zh.insert(TranslationKey::Connecting, "连接中...");
        zh.insert(TranslationKey::Error, "错误");
        zh.insert(TranslationKey::Settings, "设置");
        zh.insert(TranslationKey::Language, "语言");
        zh.insert(TranslationKey::Theme, "主题");
        zh.insert(TranslationKey::DarkMode, "深色模式");
        zh.insert(TranslationKey::LightMode, "浅色模式");
        zh.insert(TranslationKey::CompressionLevel, "压缩级别");
        zh.insert(TranslationKey::ThreadCount, "线程数");
        zh.insert(TranslationKey::BufferSize, "缓冲区大小");
        zh.insert(TranslationKey::EnableCompression, "启用 LZ4 压缩");
        zh.insert(TranslationKey::Cancel, "取消");
        zh.insert(TranslationKey::Confirm, "确认");
        zh.insert(TranslationKey::Retry, "重试");
        zh.insert(TranslationKey::Close, "关闭");
        zh.insert(TranslationKey::Browse, "浏览");
        zh.insert(TranslationKey::Clear, "清除");
        zh.insert(TranslationKey::CopyId, "复制ID");
        zh.insert(TranslationKey::Refresh, "刷新");
        zh.insert(TranslationKey::EditName, "编辑名称");
        zh.insert(TranslationKey::SaveName, "保存");
        zh.insert(TranslationKey::ConnectionFailed, "连接失败");
        zh.insert(TranslationKey::TransferComplete, "传输完成！");
        zh.insert(TranslationKey::TransferFailed, "传输失败");
        zh.insert(TranslationKey::InvalidId, "无效ID");
        zh.insert(TranslationKey::NoFilesSelected, "未选择文件");
        zh.insert(TranslationKey::RequestSent, "请求已发送");
        zh.insert(TranslationKey::RequestReceived, "收到请求");
        zh.insert(TranslationKey::RequestAccepted, "请求已接受");
        zh.insert(TranslationKey::RequestRejected, "请求被拒绝");
        zh.insert(TranslationKey::FileNotFound, "文件未找到");
        zh.insert(TranslationKey::PermissionDenied, "权限不足");
        zh.insert(TranslationKey::NetworkError, "网络错误");
        zh.insert(TranslationKey::Sender, "发送方");
        zh.insert(TranslationKey::StartReceive, "开始接收");
        zh.insert(TranslationKey::IdTooltip, "将此ID分享给发送方");
        zh.insert(TranslationKey::CompressTooltip, "启用压缩以加快传输");
        zh.insert(TranslationKey::ThreadTooltip, "更多线程 = 更快传输");
        zh.insert(TranslationKey::FileName, "文件名");
        zh.insert(TranslationKey::FileSize, "文件大小");
        zh.insert(TranslationKey::FileCount, "文件数量");
        zh.insert(TranslationKey::TotalSize, "总大小");
        zh.insert(TranslationKey::SelectFilesHint, "或将文件拖放到此处");
        zh.insert(TranslationKey::EnableEncryption, "启用传输加密");
        zh.insert(TranslationKey::EnableAnimations, "启用动画");
        zh.insert(TranslationKey::ColorTheme, "颜色主题");
        zh.insert(TranslationKey::SaveSettings, "保存设置");
        zh.insert(TranslationKey::ElectricIndigoTheme, "电光紫罗兰");
        zh.insert(TranslationKey::ClassicBlueTheme, "经典蓝");
        zh.insert(TranslationKey::DarkTheme, "深色主题");
        zh.insert(TranslationKey::LightTheme, "浅色主题");
        zh.insert(TranslationKey::OrangeCatTheme, "橘猫主题");
        data.insert(Language::Chinese, zh);

        // Spanish (Español)
        let mut es = HashMap::new();
        es.insert(TranslationKey::AppTitle, "CatShare");
        es.insert(
            TranslationKey::AppSubtitle,
            "Compartir archivos rápido y fácil",
        );
        es.insert(TranslationKey::YourName, "Tu nombre");
        es.insert(TranslationKey::YourId, "Tu ID");
        es.insert(
            TranslationKey::WaitingForConnection,
            "Esperando conexión...",
        );
        es.insert(TranslationKey::Send, "Enviar");
        es.insert(TranslationKey::SendFile, "Enviar archivo");
        es.insert(TranslationKey::SendFiles, "Enviar archivos");
        es.insert(TranslationKey::SelectFiles, "Seleccionar archivos");
        es.insert(TranslationKey::SelectedFiles, "Archivos seleccionados");
        es.insert(TranslationKey::EnterId, "Ingresar ID");
        es.insert(
            TranslationKey::EnterReceiverIdHint,
            "Ingrese el ID del receptor",
        );
        es.insert(
            TranslationKey::WaitingConfirmation,
            "Esperando confirmación...",
        );
        es.insert(TranslationKey::SendingTo, "Enviando a");
        es.insert(TranslationKey::Compress, "Comprimir");
        es.insert(TranslationKey::StartSend, "Iniciar envío");
        es.insert(TranslationKey::Receive, "Recibir");
        es.insert(TranslationKey::ReceiveFile, "Recibir archivo");
        es.insert(TranslationKey::ReceiveFiles, "Recibir archivos");
        es.insert(
            TranslationKey::WaitingForSender,
            "Esperando al remitente...",
        );
        es.insert(TranslationKey::IncomingRequest, "Solicitud entrante");
        es.insert(TranslationKey::Accept, "Aceptar");
        es.insert(TranslationKey::Reject, "Rechazar");
        es.insert(TranslationKey::ReceivingFrom, "Recibiendo de");
        es.insert(TranslationKey::SaveTo, "Guardar en");
        es.insert(TranslationKey::SelectFolder, "Seleccionar carpeta");
        es.insert(TranslationKey::Progress, "Progreso");
        es.insert(TranslationKey::Compressing, "Comprimiendo...");
        es.insert(TranslationKey::Transferring, "Transfiriendo...");
        es.insert(TranslationKey::Decompressing, "Descomprimiendo...");
        es.insert(TranslationKey::Complete, "Completo");
        es.insert(TranslationKey::Failed, "Fallido");
        es.insert(TranslationKey::Cancelled, "Cancelado");
        es.insert(TranslationKey::Speed, "Velocidad");
        es.insert(TranslationKey::TimeRemaining, "Tiempo restante");
        es.insert(TranslationKey::BytesTransferred, "Bytes transferidos");
        es.insert(TranslationKey::Connected, "Conectado");
        es.insert(TranslationKey::Disconnected, "Desconectado");
        es.insert(TranslationKey::Connecting, "Conectando...");
        es.insert(TranslationKey::Error, "Error");
        es.insert(TranslationKey::Settings, "Configuración");
        es.insert(TranslationKey::Language, "Idioma");
        es.insert(TranslationKey::Theme, "Tema");
        es.insert(TranslationKey::DarkMode, "Modo oscuro");
        es.insert(TranslationKey::LightMode, "Modo claro");
        es.insert(TranslationKey::CompressionLevel, "Nivel de compresión");
        es.insert(TranslationKey::ThreadCount, "Número de hilos");
        es.insert(TranslationKey::BufferSize, "Tamaño del búfer");
        es.insert(TranslationKey::Cancel, "Cancelar");
        es.insert(TranslationKey::Confirm, "Confirmar");
        es.insert(TranslationKey::Retry, "Reintentar");
        es.insert(TranslationKey::Close, "Cerrar");
        es.insert(TranslationKey::Browse, "Examinar");
        es.insert(TranslationKey::Clear, "Limpiar");
        es.insert(TranslationKey::CopyId, "Copiar ID");
        es.insert(TranslationKey::Refresh, "Actualizar");
        es.insert(TranslationKey::EditName, "Editar Nombre");
        es.insert(TranslationKey::SaveName, "Guardar");
        es.insert(TranslationKey::ConnectionFailed, "Conexión fallida");
        es.insert(TranslationKey::TransferComplete, "¡Transferencia completa!");
        es.insert(TranslationKey::TransferFailed, "Transferencia fallida");
        es.insert(TranslationKey::InvalidId, "ID inválido");
        es.insert(
            TranslationKey::NoFilesSelected,
            "No hay archivos seleccionados",
        );
        es.insert(TranslationKey::RequestSent, "Solicitud enviada");
        es.insert(TranslationKey::RequestReceived, "Solicitud recibida");
        es.insert(TranslationKey::RequestAccepted, "Solicitud aceptada");
        es.insert(TranslationKey::RequestRejected, "Solicitud rechazada");
        es.insert(TranslationKey::FileNotFound, "Archivo no encontrado");
        es.insert(TranslationKey::PermissionDenied, "Permiso denegado");
        es.insert(TranslationKey::NetworkError, "Error de red");
        es.insert(
            TranslationKey::IdTooltip,
            "Comparte este ID con el remitente",
        );
        es.insert(
            TranslationKey::CompressTooltip,
            "Habilita compresión para transferir más rápido",
        );
        es.insert(
            TranslationKey::ThreadTooltip,
            "Más hilos = transferencia más rápida",
        );
        es.insert(TranslationKey::FileName, "Nombre del archivo");
        es.insert(TranslationKey::FileSize, "Tamaño del archivo");
        es.insert(TranslationKey::FileCount, "Cantidad de archivos");
        es.insert(TranslationKey::TotalSize, "Tamaño total");
        es.insert(TranslationKey::AddMore, "Agregar más");
        es.insert(TranslationKey::StartReceive, "Iniciar recepción");
        es.insert(TranslationKey::Sender, "Remitente");
        es.insert(TranslationKey::Transferred, "Transferido");
        es.insert(
            TranslationKey::EnableCompression,
            "Habilitar compresión LZ4",
        );
        es.insert(
            TranslationKey::SelectFilesHint,
            "o arrastre y suelte archivos aquí",
        );
        es.insert(TranslationKey::EnableEncryption, "Habilitar cifrado de transferencia");
        es.insert(TranslationKey::EnableAnimations, "Habilitar animaciones");
        es.insert(TranslationKey::ColorTheme, "Tema de color");
        es.insert(TranslationKey::SaveSettings, "Guardar configuración");
        es.insert(TranslationKey::ElectricIndigoTheme, "Índigo Eléctrico");
        es.insert(TranslationKey::ClassicBlueTheme, "Azul Clásico");
        es.insert(TranslationKey::DarkTheme, "Tema Oscuro");
        es.insert(TranslationKey::LightTheme, "Tema Claro");
        es.insert(TranslationKey::OrangeCatTheme, "Tema Gato Naranja");
        data.insert(Language::Spanish, es);

        // Hindi (हिन्दी)
        let mut hi = HashMap::new();
        hi.insert(TranslationKey::AppTitle, "कैटशेयर");
        hi.insert(TranslationKey::AppSubtitle, "तेज़ और आसान फ़ाइल शेयरिंग");
        hi.insert(TranslationKey::YourName, "आपका नाम");
        hi.insert(TranslationKey::YourId, "आपकी ID");
        hi.insert(TranslationKey::WaitingForConnection, "कनेक्शन की प्रतीक्षा...");
        hi.insert(TranslationKey::Send, "भेजें");
        hi.insert(TranslationKey::SendFile, "फ़ाइल भेजें");
        hi.insert(TranslationKey::SendFiles, "फ़ाइलें भेजें");
        hi.insert(TranslationKey::SelectFiles, "फ़ाइलें चुनें");
        hi.insert(TranslationKey::SelectedFiles, "चयनित फ़ाइलें");
        hi.insert(TranslationKey::EnterId, "ID दर्ज करें");
        hi.insert(
            TranslationKey::EnterReceiverIdHint,
            "प्राप्तकर्ता की ID दर्ज करें",
        );
        hi.insert(TranslationKey::WaitingConfirmation, "पुष्टि की प्रतीक्षा...");
        hi.insert(TranslationKey::SendingTo, "भेज रहे हैं");
        hi.insert(TranslationKey::Compress, "संपीड़ित करें");
        hi.insert(TranslationKey::StartSend, "भेजना शुरू करें");
        hi.insert(TranslationKey::Receive, "प्राप्त करें");
        hi.insert(TranslationKey::ReceiveFile, "फ़ाइल प्राप्त करें");
        hi.insert(TranslationKey::ReceiveFiles, "फ़ाइलें प्राप्त करें");
        hi.insert(TranslationKey::WaitingForSender, "प्रेषक की प्रतीक्षा...");
        hi.insert(TranslationKey::IncomingRequest, "आने वाला अनुरोध");
        hi.insert(TranslationKey::Accept, "स्वीकार करें");
        hi.insert(TranslationKey::Reject, "अस्वीकार करें");
        hi.insert(TranslationKey::ReceivingFrom, "से प्राप्त हो रहा है");
        hi.insert(TranslationKey::SaveTo, "यहाँ सहेजें");
        hi.insert(TranslationKey::SelectFolder, "फ़ोल्डर चुनें");
        hi.insert(TranslationKey::Progress, "प्रगति");
        hi.insert(TranslationKey::Compressing, "संपीड़ित हो रहा है...");
        hi.insert(TranslationKey::Transferring, "स्थानांतरित हो रहा है...");
        hi.insert(TranslationKey::Decompressing, "विसंपीड़ित हो रहा है...");
        hi.insert(TranslationKey::Complete, "पूर्ण");
        hi.insert(TranslationKey::Failed, "विफल");
        hi.insert(TranslationKey::Cancelled, "रद्द");
        hi.insert(TranslationKey::Speed, "गति");
        hi.insert(TranslationKey::TimeRemaining, "शेष समय");
        hi.insert(TranslationKey::BytesTransferred, "बाइट्स स्थानांतरित");
        hi.insert(TranslationKey::Connected, "जुड़ा हुआ");
        hi.insert(TranslationKey::Disconnected, "डिस्कनेक्ट");
        hi.insert(TranslationKey::Connecting, "जुड़ रहा है...");
        hi.insert(TranslationKey::Error, "त्रुटि");
        hi.insert(TranslationKey::Settings, "सेटिंग्स");
        hi.insert(TranslationKey::Language, "भाषा");
        hi.insert(TranslationKey::Theme, "थीम");
        hi.insert(TranslationKey::DarkMode, "डार्क मोड");
        hi.insert(TranslationKey::LightMode, "लाइट मोड");
        hi.insert(TranslationKey::CompressionLevel, "संपीड़न स्तर");
        hi.insert(TranslationKey::ThreadCount, "थ्रेड संख्या");
        hi.insert(TranslationKey::BufferSize, "बफर आकार");
        hi.insert(TranslationKey::Cancel, "रद्द करें");
        hi.insert(TranslationKey::Confirm, "पुष्टि करें");
        hi.insert(TranslationKey::Retry, "पुनः प्रयास");
        hi.insert(TranslationKey::Close, "बंद करें");
        hi.insert(TranslationKey::Browse, "ब्राउज़ करें");
        hi.insert(TranslationKey::Clear, "साफ़ करें");
        hi.insert(TranslationKey::CopyId, "ID कॉपी करें");
        hi.insert(TranslationKey::Refresh, "रीफ्रेश");
        hi.insert(TranslationKey::EditName, "नाम संपादित करें");
        hi.insert(TranslationKey::SaveName, "सहेजें");
        hi.insert(TranslationKey::ConnectionFailed, "कनेक्शन विफल");
        hi.insert(TranslationKey::TransferComplete, "स्थानांतरण पूर्ण!");
        hi.insert(TranslationKey::TransferFailed, "स्थानांतरण विफल");
        hi.insert(TranslationKey::InvalidId, "अमान्य ID");
        hi.insert(TranslationKey::NoFilesSelected, "कोई फ़ाइल नहीं चुनी गई");
        hi.insert(TranslationKey::RequestSent, "अनुरोध भेजा गया");
        hi.insert(TranslationKey::RequestReceived, "अनुरोध प्राप्त हुआ");
        hi.insert(TranslationKey::RequestAccepted, "अनुरोध स्वीकृत");
        hi.insert(TranslationKey::RequestRejected, "अनुरोध अस्वीकृत");
        hi.insert(TranslationKey::FileNotFound, "फ़ाइल नहीं मिली");
        hi.insert(TranslationKey::PermissionDenied, "अनुमति अस्वीकृत");
        hi.insert(TranslationKey::NetworkError, "नेटवर्क त्रुटि");
        hi.insert(TranslationKey::IdTooltip, "यह ID प्रेषक को दें");
        hi.insert(
            TranslationKey::CompressTooltip,
            "तेज़ स्थानांतरण के लिए संपीड़न सक्षम करें",
        );
        hi.insert(TranslationKey::ThreadTooltip, "अधिक थ्रेड = तेज़ स्थानांतरण");
        hi.insert(TranslationKey::FileName, "फ़ाइल का नाम");
        hi.insert(TranslationKey::FileSize, "फ़ाइल का आकार");
        hi.insert(TranslationKey::FileCount, "फ़ाइल संख्या");
        hi.insert(TranslationKey::TotalSize, "कुल आकार");
        hi.insert(TranslationKey::AddMore, "और जोड़ें");
        hi.insert(TranslationKey::StartReceive, "प्राप्त करना शुरू करें");
        hi.insert(TranslationKey::Sender, "प्रेषक");
        hi.insert(TranslationKey::Transferred, "स्थानांतरित");
        hi.insert(TranslationKey::EnableCompression, "LZ4 संपीड़न सक्षम करें");
        hi.insert(
            TranslationKey::SelectFilesHint,
            "या फ़ाइलों को यहाँ खींचें और छोड़ें",
        );
        hi.insert(TranslationKey::EnableEncryption, "स्थानांतरण एन्क्रिप्शन सक्षम करें");
        hi.insert(TranslationKey::EnableAnimations, "एनीमेशन सक्षम करें");
        hi.insert(TranslationKey::ColorTheme, "रंग थीम");
        hi.insert(TranslationKey::SaveSettings, "सेटिंग्स सहेजें");
        hi.insert(TranslationKey::ElectricIndigoTheme, "इलेक्ट्रिक इंडिगो");
        hi.insert(TranslationKey::ClassicBlueTheme, "क्लासिक ब्लू");
        hi.insert(TranslationKey::DarkTheme, "डार्क थीम");
        hi.insert(TranslationKey::LightTheme, "लाइट थीम");
        hi.insert(TranslationKey::OrangeCatTheme, "ऑरेंज कैट थीम");
        data.insert(Language::Hindi, hi);

        // Arabic (العربية)
        let mut ar = HashMap::new();
        ar.insert(TranslationKey::AppTitle, "كات شير");
        ar.insert(TranslationKey::AppSubtitle, "مشاركة الملفات بسرعة وسهولة");
        ar.insert(TranslationKey::YourName, "اسمك");
        ar.insert(TranslationKey::YourId, "معرفك");
        ar.insert(TranslationKey::WaitingForConnection, "في انتظار الاتصال...");
        ar.insert(TranslationKey::Send, "إرسال");
        ar.insert(TranslationKey::SendFile, "إرسال ملف");
        ar.insert(TranslationKey::SendFiles, "إرسال ملفات");
        ar.insert(TranslationKey::SelectFiles, "اختيار الملفات");
        ar.insert(TranslationKey::SelectedFiles, "الملفات المختارة");
        ar.insert(TranslationKey::EnterId, "أدخل المعرف");
        ar.insert(TranslationKey::EnterReceiverIdHint, "أدخل معرف المستلم");
        ar.insert(TranslationKey::WaitingConfirmation, "في انتظار التأكيد...");
        ar.insert(TranslationKey::SendingTo, "إرسال إلى");
        ar.insert(TranslationKey::Compress, "ضغط");
        ar.insert(TranslationKey::StartSend, "بدء الإرسال");
        ar.insert(TranslationKey::Receive, "استلام");
        ar.insert(TranslationKey::ReceiveFile, "استلام ملف");
        ar.insert(TranslationKey::ReceiveFiles, "استلام ملفات");
        ar.insert(TranslationKey::WaitingForSender, "في انتظار المرسل...");
        ar.insert(TranslationKey::IncomingRequest, "طلب وارد");
        ar.insert(TranslationKey::Accept, "قبول");
        ar.insert(TranslationKey::Reject, "رفض");
        ar.insert(TranslationKey::ReceivingFrom, "استلام من");
        ar.insert(TranslationKey::SaveTo, "حفظ في");
        ar.insert(TranslationKey::SelectFolder, "اختيار مجلد");
        ar.insert(TranslationKey::Progress, "التقدم");
        ar.insert(TranslationKey::Compressing, "جاري الضغط...");
        ar.insert(TranslationKey::Transferring, "جاري النقل...");
        ar.insert(TranslationKey::Decompressing, "جاري فك الضغط...");
        ar.insert(TranslationKey::Complete, "مكتمل");
        ar.insert(TranslationKey::Failed, "فشل");
        ar.insert(TranslationKey::Cancelled, "ملغي");
        ar.insert(TranslationKey::Speed, "السرعة");
        ar.insert(TranslationKey::TimeRemaining, "الوقت المتبقي");
        ar.insert(TranslationKey::BytesTransferred, "البايتات المنقولة");
        ar.insert(TranslationKey::Connected, "متصل");
        ar.insert(TranslationKey::Disconnected, "غير متصل");
        ar.insert(TranslationKey::Connecting, "جاري الاتصال...");
        ar.insert(TranslationKey::Error, "خطأ");
        ar.insert(TranslationKey::Settings, "الإعدادات");
        ar.insert(TranslationKey::Language, "اللغة");
        ar.insert(TranslationKey::Theme, "المظهر");
        ar.insert(TranslationKey::DarkMode, "الوضع الداكن");
        ar.insert(TranslationKey::LightMode, "الوضع الفاتح");
        ar.insert(TranslationKey::CompressionLevel, "مستوى الضغط");
        ar.insert(TranslationKey::ThreadCount, "عدد الخيوط");
        ar.insert(TranslationKey::BufferSize, "حجم المخزن المؤقت");
        ar.insert(TranslationKey::Cancel, "إلغاء");
        ar.insert(TranslationKey::Confirm, "تأكيد");
        ar.insert(TranslationKey::Retry, "إعادة المحاولة");
        ar.insert(TranslationKey::Close, "إغلاق");
        ar.insert(TranslationKey::Browse, "تصفح");
        ar.insert(TranslationKey::Clear, "مسح");
        ar.insert(TranslationKey::CopyId, "نسخ المعرف");
        ar.insert(TranslationKey::Refresh, "تحديث");
        ar.insert(TranslationKey::EditName, "تعديل الاسم");
        ar.insert(TranslationKey::SaveName, "حفظ");
        ar.insert(TranslationKey::ConnectionFailed, "فشل الاتصال");
        ar.insert(TranslationKey::TransferComplete, "اكتمل النقل!");
        ar.insert(TranslationKey::TransferFailed, "فشل النقل");
        ar.insert(TranslationKey::InvalidId, "معرف غير صالح");
        ar.insert(TranslationKey::NoFilesSelected, "لم يتم اختيار ملفات");
        ar.insert(TranslationKey::RequestSent, "تم إرسال الطلب");
        ar.insert(TranslationKey::RequestReceived, "تم استلام الطلب");
        ar.insert(TranslationKey::RequestAccepted, "تم قبول الطلب");
        ar.insert(TranslationKey::RequestRejected, "تم رفض الطلب");
        ar.insert(TranslationKey::FileNotFound, "الملف غير موجود");
        ar.insert(TranslationKey::PermissionDenied, "تم رفض الإذن");
        ar.insert(TranslationKey::NetworkError, "خطأ في الشبكة");
        ar.insert(TranslationKey::IdTooltip, "شارك هذا المعرف مع المرسل");
        ar.insert(TranslationKey::CompressTooltip, "تفعيل الضغط لنقل أسرع");
        ar.insert(TranslationKey::ThreadTooltip, "خيوط أكثر = نقل أسرع");
        ar.insert(TranslationKey::FileName, "اسم الملف");
        ar.insert(TranslationKey::FileSize, "حجم الملف");
        ar.insert(TranslationKey::FileCount, "عدد الملفات");
        ar.insert(TranslationKey::TotalSize, "الحجم الإجمالي");
        ar.insert(TranslationKey::AddMore, "إضافة المزيد");
        ar.insert(TranslationKey::StartReceive, "بدء الاستلام");
        ar.insert(TranslationKey::Sender, "المرسل");
        ar.insert(TranslationKey::Transferred, "تم نقله");
        ar.insert(TranslationKey::EnableCompression, "تفعيل ضغط LZ4");
        ar.insert(
            TranslationKey::SelectFilesHint,
            "أو قم بسحب وإفلات الملفات هنا",
        );
        ar.insert(TranslationKey::EnableEncryption, "تمكين تشفير النقل");
        ar.insert(TranslationKey::EnableAnimations, "تمكين الرسوم المتحركة");
        ar.insert(TranslationKey::ColorTheme, "سمة الألوان");
        ar.insert(TranslationKey::SaveSettings, "حفظ الإعدادات");
        ar.insert(TranslationKey::ElectricIndigoTheme, "اللون البنفسجي الكهربائي");
        ar.insert(TranslationKey::ClassicBlueTheme, "الأزرق الكلاسيكي");
        ar.insert(TranslationKey::DarkTheme, "السمة الداكنة");
        ar.insert(TranslationKey::LightTheme, "السمة الفاتحة");
        ar.insert(TranslationKey::OrangeCatTheme, "سمة القطة البرتقالية");
        data.insert(Language::Arabic, ar);

        // Portuguese (Português)
        let mut pt = HashMap::new();
        pt.insert(TranslationKey::AppTitle, "CatShare");
        pt.insert(
            TranslationKey::AppSubtitle,
            "Compartilhamento de arquivos rápido e fácil",
        );
        pt.insert(TranslationKey::YourName, "Seu nome");
        pt.insert(TranslationKey::YourId, "Seu ID");
        pt.insert(
            TranslationKey::WaitingForConnection,
            "Aguardando conexão...",
        );
        pt.insert(TranslationKey::Send, "Enviar");
        pt.insert(TranslationKey::SendFile, "Enviar arquivo");
        pt.insert(TranslationKey::SendFiles, "Enviar arquivos");
        pt.insert(TranslationKey::SelectFiles, "Selecionar arquivos");
        pt.insert(TranslationKey::SelectedFiles, "Arquivos selecionados");
        pt.insert(TranslationKey::EnterId, "Digite o ID");
        pt.insert(
            TranslationKey::EnterReceiverIdHint,
            "Digite o ID do receptor",
        );
        pt.insert(
            TranslationKey::WaitingConfirmation,
            "Aguardando confirmação...",
        );
        pt.insert(TranslationKey::SendingTo, "Enviando para");
        pt.insert(TranslationKey::Compress, "Comprimir");
        pt.insert(TranslationKey::StartSend, "Iniciar envio");
        pt.insert(TranslationKey::Receive, "Receber");
        pt.insert(TranslationKey::ReceiveFile, "Receber arquivo");
        pt.insert(TranslationKey::ReceiveFiles, "Receber arquivos");
        pt.insert(TranslationKey::WaitingForSender, "Aguardando remetente...");
        pt.insert(TranslationKey::IncomingRequest, "Solicitação recebida");
        pt.insert(TranslationKey::Accept, "Aceitar");
        pt.insert(TranslationKey::Reject, "Rejeitar");
        pt.insert(TranslationKey::ReceivingFrom, "Recebendo de");
        pt.insert(TranslationKey::SaveTo, "Salvar em");
        pt.insert(TranslationKey::SelectFolder, "Selecionar pasta");
        pt.insert(TranslationKey::Progress, "Progresso");
        pt.insert(TranslationKey::Compressing, "Comprimindo...");
        pt.insert(TranslationKey::Transferring, "Transferindo...");
        pt.insert(TranslationKey::Decompressing, "Descomprimindo...");
        pt.insert(TranslationKey::Complete, "Completo");
        pt.insert(TranslationKey::Failed, "Falhou");
        pt.insert(TranslationKey::Cancelled, "Cancelado");
        pt.insert(TranslationKey::Speed, "Velocidade");
        pt.insert(TranslationKey::TimeRemaining, "Tempo restante");
        pt.insert(TranslationKey::BytesTransferred, "Bytes transferidos");
        pt.insert(TranslationKey::Connected, "Conectado");
        pt.insert(TranslationKey::Disconnected, "Desconectado");
        pt.insert(TranslationKey::Connecting, "Conectando...");
        pt.insert(TranslationKey::Error, "Erro");
        pt.insert(TranslationKey::Settings, "Configurações");
        pt.insert(TranslationKey::Language, "Idioma");
        pt.insert(TranslationKey::Theme, "Tema");
        pt.insert(TranslationKey::DarkMode, "Modo escuro");
        pt.insert(TranslationKey::LightMode, "Modo claro");
        pt.insert(TranslationKey::CompressionLevel, "Nível de compressão");
        pt.insert(TranslationKey::ThreadCount, "Número de threads");
        pt.insert(TranslationKey::BufferSize, "Tamanho do buffer");
        pt.insert(TranslationKey::Cancel, "Cancelar");
        pt.insert(TranslationKey::Confirm, "Confirmar");
        pt.insert(TranslationKey::Retry, "Tentar novamente");
        pt.insert(TranslationKey::Close, "Fechar");
        pt.insert(TranslationKey::Browse, "Procurar");
        pt.insert(TranslationKey::Clear, "Limpar");
        pt.insert(TranslationKey::CopyId, "Copiar ID");
        pt.insert(TranslationKey::Refresh, "Atualizar");
        pt.insert(TranslationKey::EditName, "Editar Nome");
        pt.insert(TranslationKey::SaveName, "Salvar");
        pt.insert(TranslationKey::ConnectionFailed, "Conexão falhou");
        pt.insert(TranslationKey::TransferComplete, "Transferência completa!");
        pt.insert(TranslationKey::TransferFailed, "Transferência falhou");
        pt.insert(TranslationKey::InvalidId, "ID inválido");
        pt.insert(
            TranslationKey::NoFilesSelected,
            "Nenhum arquivo selecionado",
        );
        pt.insert(TranslationKey::RequestSent, "Solicitação enviada");
        pt.insert(TranslationKey::RequestReceived, "Solicitação recebida");
        pt.insert(TranslationKey::RequestAccepted, "Solicitação aceita");
        pt.insert(TranslationKey::RequestRejected, "Solicitação rejeitada");
        pt.insert(TranslationKey::FileNotFound, "Arquivo não encontrado");
        pt.insert(TranslationKey::PermissionDenied, "Permissão negada");
        pt.insert(TranslationKey::NetworkError, "Erro de rede");
        pt.insert(
            TranslationKey::IdTooltip,
            "Compartilhe este ID com o remetente",
        );
        pt.insert(
            TranslationKey::CompressTooltip,
            "Ativar compressão para transferência mais rápida",
        );
        pt.insert(
            TranslationKey::ThreadTooltip,
            "Mais threads = transferência mais rápida",
        );
        pt.insert(TranslationKey::FileName, "Nome do arquivo");
        pt.insert(TranslationKey::FileSize, "Tamanho do arquivo");
        pt.insert(TranslationKey::FileCount, "Quantidade de arquivos");
        pt.insert(TranslationKey::TotalSize, "Tamanho total");
        pt.insert(TranslationKey::AddMore, "Adicionar mais");
        pt.insert(TranslationKey::StartReceive, "Iniciar recepção");
        pt.insert(TranslationKey::Sender, "Remetente");
        pt.insert(TranslationKey::Transferred, "Transferido");
        pt.insert(TranslationKey::EnableCompression, "Ativar compressão LZ4");
        pt.insert(
            TranslationKey::SelectFilesHint,
            "ou arraste e solte arquivos aqui",
        );
        pt.insert(TranslationKey::EnableEncryption, "Ativar criptografia de transferência");
        pt.insert(TranslationKey::EnableAnimations, "Ativar animações");
        pt.insert(TranslationKey::ColorTheme, "Tema de cores");
        pt.insert(TranslationKey::SaveSettings, "Salvar configurações");
        pt.insert(TranslationKey::ElectricIndigoTheme, "Índigo Elétrico");
        pt.insert(TranslationKey::ClassicBlueTheme, "Azul Clássico");
        pt.insert(TranslationKey::DarkTheme, "Tema Escuro");
        pt.insert(TranslationKey::LightTheme, "Tema Claro");
        pt.insert(TranslationKey::OrangeCatTheme, "Tema Gato Laranja");
        data.insert(Language::Portuguese, pt);

        // Russian (Русский)
        let mut ru = HashMap::new();
        ru.insert(TranslationKey::AppTitle, "CatShare");
        ru.insert(
            TranslationKey::AppSubtitle,
            "Быстрый и простой обмен файлами",
        );
        ru.insert(TranslationKey::YourName, "Ваше имя");
        ru.insert(TranslationKey::YourId, "Ваш ID");
        ru.insert(
            TranslationKey::WaitingForConnection,
            "Ожидание подключения...",
        );
        ru.insert(TranslationKey::Send, "Отправить");
        ru.insert(TranslationKey::SendFile, "Отправить файл");
        ru.insert(TranslationKey::SendFiles, "Отправить файлы");
        ru.insert(TranslationKey::SelectFiles, "Выбрать файлы");
        ru.insert(TranslationKey::SelectedFiles, "Выбранные файлы");
        ru.insert(TranslationKey::EnterId, "Введите ID");
        ru.insert(TranslationKey::EnterReceiverIdHint, "Введите ID получателя");
        ru.insert(
            TranslationKey::WaitingConfirmation,
            "Ожидание подтверждения...",
        );
        ru.insert(TranslationKey::SendingTo, "Отправка");
        ru.insert(TranslationKey::Compress, "Сжатие");
        ru.insert(TranslationKey::StartSend, "Начать отправку");
        ru.insert(TranslationKey::Receive, "Получить");
        ru.insert(TranslationKey::ReceiveFile, "Получить файл");
        ru.insert(TranslationKey::ReceiveFiles, "Получить файлы");
        ru.insert(TranslationKey::WaitingForSender, "Ожидание отправителя...");
        ru.insert(TranslationKey::IncomingRequest, "Входящий запрос");
        ru.insert(TranslationKey::Accept, "Принять");
        ru.insert(TranslationKey::Reject, "Отклонить");
        ru.insert(TranslationKey::ReceivingFrom, "Получение от");
        ru.insert(TranslationKey::SaveTo, "Сохранить в");
        ru.insert(TranslationKey::SelectFolder, "Выбрать папку");
        ru.insert(TranslationKey::Progress, "Прогресс");
        ru.insert(TranslationKey::Compressing, "Сжатие...");
        ru.insert(TranslationKey::Transferring, "Передача...");
        ru.insert(TranslationKey::Decompressing, "Распаковка...");
        ru.insert(TranslationKey::Complete, "Завершено");
        ru.insert(TranslationKey::Failed, "Ошибка");
        ru.insert(TranslationKey::Cancelled, "Отменено");
        ru.insert(TranslationKey::Speed, "Скорость");
        ru.insert(TranslationKey::TimeRemaining, "Осталось времени");
        ru.insert(TranslationKey::BytesTransferred, "Передано байт");
        ru.insert(TranslationKey::Connected, "Подключено");
        ru.insert(TranslationKey::Disconnected, "Отключено");
        ru.insert(TranslationKey::Connecting, "Подключение...");
        ru.insert(TranslationKey::Error, "Ошибка");
        ru.insert(TranslationKey::Settings, "Настройки");
        ru.insert(TranslationKey::Language, "Язык");
        ru.insert(TranslationKey::Theme, "Тема");
        ru.insert(TranslationKey::DarkMode, "Тёмный режим");
        ru.insert(TranslationKey::LightMode, "Светлый режим");
        ru.insert(TranslationKey::CompressionLevel, "Уровень сжатия");
        ru.insert(TranslationKey::ThreadCount, "Количество потоков");
        ru.insert(TranslationKey::BufferSize, "Размер буфера");
        ru.insert(TranslationKey::Cancel, "Отмена");
        ru.insert(TranslationKey::Confirm, "Подтвердить");
        ru.insert(TranslationKey::Retry, "Повторить");
        ru.insert(TranslationKey::Close, "Закрыть");
        ru.insert(TranslationKey::Browse, "Обзор");
        ru.insert(TranslationKey::Clear, "Очистить");
        ru.insert(TranslationKey::CopyId, "Копировать ID");
        ru.insert(TranslationKey::Refresh, "Обновить");
        ru.insert(TranslationKey::EditName, "Изменить имя");
        ru.insert(TranslationKey::SaveName, "Сохранить");
        ru.insert(TranslationKey::ConnectionFailed, "Ошибка подключения");
        ru.insert(TranslationKey::TransferComplete, "Передача завершена!");
        ru.insert(TranslationKey::TransferFailed, "Ошибка передачи");
        ru.insert(TranslationKey::InvalidId, "Неверный ID");
        ru.insert(TranslationKey::NoFilesSelected, "Файлы не выбраны");
        ru.insert(TranslationKey::RequestSent, "Запрос отправлен");
        ru.insert(TranslationKey::RequestReceived, "Запрос получен");
        ru.insert(TranslationKey::RequestAccepted, "Запрос принят");
        ru.insert(TranslationKey::RequestRejected, "Запрос отклонён");
        ru.insert(TranslationKey::FileNotFound, "Файл не найден");
        ru.insert(TranslationKey::PermissionDenied, "Доступ запрещён");
        ru.insert(TranslationKey::NetworkError, "Ошибка сети");
        ru.insert(
            TranslationKey::IdTooltip,
            "Поделитесь этим ID с отправителем",
        );
        ru.insert(
            TranslationKey::CompressTooltip,
            "Включить сжатие для быстрой передачи",
        );
        ru.insert(
            TranslationKey::ThreadTooltip,
            "Больше потоков = быстрее передача",
        );
        ru.insert(TranslationKey::FileName, "Имя файла");
        ru.insert(TranslationKey::FileSize, "Размер файла");
        ru.insert(TranslationKey::FileCount, "Количество файлов");
        ru.insert(TranslationKey::TotalSize, "Общий размер");
        ru.insert(TranslationKey::AddMore, "Добавить еще");
        ru.insert(TranslationKey::StartReceive, "Начать прием");
        ru.insert(TranslationKey::Sender, "Отправитель");
        ru.insert(TranslationKey::Transferred, "Передано");
        ru.insert(TranslationKey::EnableCompression, "Включить сжатие LZ4");
        ru.insert(TranslationKey::SelectFilesHint, "или перетащите файлы сюда");
        ru.insert(TranslationKey::EnableEncryption, "Включить шифрование передачи");
        ru.insert(TranslationKey::EnableAnimations, "Включить анимации");
        ru.insert(TranslationKey::ColorTheme, "Цветовая тема");
        ru.insert(TranslationKey::SaveSettings, "Сохранить настройки");
        ru.insert(TranslationKey::ElectricIndigoTheme, "Электрический индиго");
        ru.insert(TranslationKey::ClassicBlueTheme, "Классический синий");
        ru.insert(TranslationKey::DarkTheme, "Темная тема");
        ru.insert(TranslationKey::LightTheme, "Светлая тема");
        ru.insert(TranslationKey::OrangeCatTheme, "Тема оранжевого кота");
        data.insert(Language::Russian, ru);

        // Japanese (日本語)
        let mut ja = HashMap::new();
        ja.insert(TranslationKey::AppTitle, "キャットシェア");
        ja.insert(TranslationKey::AppSubtitle, "高速で簡単なファイル共有");
        ja.insert(TranslationKey::YourName, "あなたの名前");
        ja.insert(TranslationKey::YourId, "あなたのID");
        ja.insert(TranslationKey::WaitingForConnection, "接続待機中...");
        ja.insert(TranslationKey::Send, "送信");
        ja.insert(TranslationKey::SendFile, "ファイルを送信");
        ja.insert(TranslationKey::SendFiles, "ファイルを送信");
        ja.insert(TranslationKey::SelectFiles, "ファイルを選択");
        ja.insert(TranslationKey::SelectedFiles, "選択されたファイル");
        ja.insert(TranslationKey::EnterId, "IDを入力");
        ja.insert(TranslationKey::EnterReceiverIdHint, "受信者のIDを入力");
        ja.insert(TranslationKey::WaitingConfirmation, "確認待ち...");
        ja.insert(TranslationKey::SendingTo, "送信先");
        ja.insert(TranslationKey::Compress, "圧縮");
        ja.insert(TranslationKey::StartSend, "送信開始");
        ja.insert(TranslationKey::Receive, "受信");
        ja.insert(TranslationKey::ReceiveFile, "ファイルを受信");
        ja.insert(TranslationKey::ReceiveFiles, "ファイルを受信");
        ja.insert(TranslationKey::WaitingForSender, "送信者を待機中...");
        ja.insert(TranslationKey::IncomingRequest, "受信リクエスト");
        ja.insert(TranslationKey::Accept, "承諾");
        ja.insert(TranslationKey::Reject, "拒否");
        ja.insert(TranslationKey::ReceivingFrom, "受信元");
        ja.insert(TranslationKey::SaveTo, "保存先");
        ja.insert(TranslationKey::SelectFolder, "フォルダを選択");
        ja.insert(TranslationKey::Progress, "進捗");
        ja.insert(TranslationKey::Compressing, "圧縮中...");
        ja.insert(TranslationKey::Transferring, "転送中...");
        ja.insert(TranslationKey::Decompressing, "解凍中...");
        ja.insert(TranslationKey::Complete, "完了");
        ja.insert(TranslationKey::Failed, "失敗");
        ja.insert(TranslationKey::Cancelled, "キャンセル");
        ja.insert(TranslationKey::Speed, "速度");
        ja.insert(TranslationKey::TimeRemaining, "残り時間");
        ja.insert(TranslationKey::BytesTransferred, "転送済みバイト");
        ja.insert(TranslationKey::Connected, "接続済み");
        ja.insert(TranslationKey::Disconnected, "切断");
        ja.insert(TranslationKey::Connecting, "接続中...");
        ja.insert(TranslationKey::Error, "エラー");
        ja.insert(TranslationKey::Settings, "設定");
        ja.insert(TranslationKey::Language, "言語");
        ja.insert(TranslationKey::Theme, "テーマ");
        ja.insert(TranslationKey::DarkMode, "ダークモード");
        ja.insert(TranslationKey::LightMode, "ライトモード");
        ja.insert(TranslationKey::CompressionLevel, "圧縮レベル");
        ja.insert(TranslationKey::ThreadCount, "スレッド数");
        ja.insert(TranslationKey::BufferSize, "バッファサイズ");
        ja.insert(TranslationKey::Cancel, "キャンセル");
        ja.insert(TranslationKey::Confirm, "確認");
        ja.insert(TranslationKey::Retry, "再試行");
        ja.insert(TranslationKey::Close, "閉じる");
        ja.insert(TranslationKey::Browse, "参照");
        ja.insert(TranslationKey::Clear, "クリア");
        ja.insert(TranslationKey::CopyId, "IDをコピー");
        ja.insert(TranslationKey::Refresh, "更新");
        ja.insert(TranslationKey::EditName, "名前を編集");
        ja.insert(TranslationKey::SaveName, "保存");
        ja.insert(TranslationKey::ConnectionFailed, "接続失敗");
        ja.insert(TranslationKey::TransferComplete, "転送完了！");
        ja.insert(TranslationKey::TransferFailed, "転送失敗");
        ja.insert(TranslationKey::InvalidId, "無効なID");
        ja.insert(
            TranslationKey::NoFilesSelected,
            "ファイルが選択されていません",
        );
        ja.insert(TranslationKey::RequestSent, "リクエスト送信済み");
        ja.insert(TranslationKey::RequestReceived, "リクエスト受信");
        ja.insert(TranslationKey::RequestAccepted, "リクエスト承諾");
        ja.insert(TranslationKey::RequestRejected, "リクエスト拒否");
        ja.insert(TranslationKey::FileNotFound, "ファイルが見つかりません");
        ja.insert(TranslationKey::PermissionDenied, "権限がありません");
        ja.insert(TranslationKey::NetworkError, "ネットワークエラー");
        ja.insert(
            TranslationKey::IdTooltip,
            "このIDを送信者に共有してください",
        );
        ja.insert(
            TranslationKey::CompressTooltip,
            "高速転送のため圧縮を有効にする",
        );
        ja.insert(
            TranslationKey::ThreadTooltip,
            "スレッド数が多い = 転送が速い",
        );
        ja.insert(TranslationKey::FileName, "ファイル名");
        ja.insert(TranslationKey::FileSize, "ファイルサイズ");
        ja.insert(TranslationKey::FileCount, "ファイル数");
        ja.insert(TranslationKey::TotalSize, "合計サイズ");
        ja.insert(TranslationKey::AddMore, "さらに追加");
        ja.insert(TranslationKey::StartReceive, "受信開始");
        ja.insert(TranslationKey::Sender, "送信者");
        ja.insert(TranslationKey::Transferred, "転送済み");
        ja.insert(TranslationKey::EnableCompression, "LZ4圧縮を有効にする");
        ja.insert(
            TranslationKey::SelectFilesHint,
            "またはファイルをここにドラッグ＆ドロップ",
        );
        ja.insert(TranslationKey::EnableEncryption, "転送暗号化を有効にする");
        ja.insert(TranslationKey::EnableAnimations, "アニメーションを有効にする");
        ja.insert(TranslationKey::ColorTheme, "カラーテーマ");
        ja.insert(TranslationKey::SaveSettings, "設定を保存");
        ja.insert(TranslationKey::ElectricIndigoTheme, "エレクトリックインディゴ");
        ja.insert(TranslationKey::ClassicBlueTheme, "クラシックブルー");
        ja.insert(TranslationKey::DarkTheme, "ダークテーマ");
        ja.insert(TranslationKey::LightTheme, "ライトテーマ");
        ja.insert(TranslationKey::OrangeCatTheme, "オレンジキャットテーマ");
        data.insert(Language::Japanese, ja);

        // German (Deutsch)
        let mut de = HashMap::new();
        de.insert(TranslationKey::AppTitle, "CatShare");
        de.insert(
            TranslationKey::AppSubtitle,
            "Schneller und einfacher Dateiaustausch",
        );
        de.insert(TranslationKey::YourName, "Ihr Name");
        de.insert(TranslationKey::YourId, "Ihre ID");
        de.insert(
            TranslationKey::WaitingForConnection,
            "Warte auf Verbindung...",
        );
        de.insert(TranslationKey::Send, "Senden");
        de.insert(TranslationKey::SendFile, "Datei senden");
        de.insert(TranslationKey::SendFiles, "Dateien senden");
        de.insert(TranslationKey::SelectFiles, "Dateien auswählen");
        de.insert(TranslationKey::SelectedFiles, "Ausgewählte Dateien");
        de.insert(TranslationKey::EnterId, "ID eingeben");
        de.insert(TranslationKey::EnterReceiverIdHint, "Empfänger-ID eingeben");
        de.insert(
            TranslationKey::WaitingConfirmation,
            "Warte auf Bestätigung...",
        );
        de.insert(TranslationKey::SendingTo, "Senden an");
        de.insert(TranslationKey::Compress, "Komprimieren");
        de.insert(TranslationKey::StartSend, "Senden starten");
        de.insert(TranslationKey::Receive, "Empfangen");
        de.insert(TranslationKey::ReceiveFile, "Datei empfangen");
        de.insert(TranslationKey::ReceiveFiles, "Dateien empfangen");
        de.insert(TranslationKey::WaitingForSender, "Warte auf Absender...");
        de.insert(TranslationKey::IncomingRequest, "Eingehende Anfrage");
        de.insert(TranslationKey::Accept, "Akzeptieren");
        de.insert(TranslationKey::Reject, "Ablehnen");
        de.insert(TranslationKey::ReceivingFrom, "Empfange von");
        de.insert(TranslationKey::SaveTo, "Speichern unter");
        de.insert(TranslationKey::SelectFolder, "Ordner auswählen");
        de.insert(TranslationKey::Progress, "Fortschritt");
        de.insert(TranslationKey::Compressing, "Komprimiere...");
        de.insert(TranslationKey::Transferring, "Übertrage...");
        de.insert(TranslationKey::Decompressing, "Dekomprimiere...");
        de.insert(TranslationKey::Complete, "Abgeschlossen");
        de.insert(TranslationKey::Failed, "Fehlgeschlagen");
        de.insert(TranslationKey::Cancelled, "Abgebrochen");
        de.insert(TranslationKey::Speed, "Geschwindigkeit");
        de.insert(TranslationKey::TimeRemaining, "Verbleibende Zeit");
        de.insert(TranslationKey::BytesTransferred, "Übertragene Bytes");
        de.insert(TranslationKey::Connected, "Verbunden");
        de.insert(TranslationKey::Disconnected, "Getrennt");
        de.insert(TranslationKey::Connecting, "Verbinde...");
        de.insert(TranslationKey::Error, "Fehler");
        de.insert(TranslationKey::Settings, "Einstellungen");
        de.insert(TranslationKey::Language, "Sprache");
        de.insert(TranslationKey::Theme, "Design");
        de.insert(TranslationKey::DarkMode, "Dunkelmodus");
        de.insert(TranslationKey::LightMode, "Hellmodus");
        de.insert(TranslationKey::CompressionLevel, "Komprimierungsstufe");
        de.insert(TranslationKey::ThreadCount, "Thread-Anzahl");
        de.insert(TranslationKey::BufferSize, "Puffergröße");
        de.insert(TranslationKey::Cancel, "Abbrechen");
        de.insert(TranslationKey::Confirm, "Bestätigen");
        de.insert(TranslationKey::Retry, "Wiederholen");
        de.insert(TranslationKey::Close, "Schließen");
        de.insert(TranslationKey::Browse, "Durchsuchen");
        de.insert(TranslationKey::Clear, "Löschen");
        de.insert(TranslationKey::CopyId, "ID kopieren");
        de.insert(TranslationKey::Refresh, "Aktualisieren");
        de.insert(TranslationKey::EditName, "Name bearbeiten");
        de.insert(TranslationKey::SaveName, "Speichern");
        de.insert(
            TranslationKey::ConnectionFailed,
            "Verbindung fehlgeschlagen",
        );
        de.insert(
            TranslationKey::TransferComplete,
            "Übertragung abgeschlossen!",
        );
        de.insert(TranslationKey::TransferFailed, "Übertragung fehlgeschlagen");
        de.insert(TranslationKey::InvalidId, "Ungültige ID");
        de.insert(TranslationKey::NoFilesSelected, "Keine Dateien ausgewählt");
        de.insert(TranslationKey::RequestSent, "Anfrage gesendet");
        de.insert(TranslationKey::RequestReceived, "Anfrage empfangen");
        de.insert(TranslationKey::RequestAccepted, "Anfrage akzeptiert");
        de.insert(TranslationKey::RequestRejected, "Anfrage abgelehnt");
        de.insert(TranslationKey::FileNotFound, "Datei nicht gefunden");
        de.insert(TranslationKey::PermissionDenied, "Zugriff verweigert");
        de.insert(TranslationKey::NetworkError, "Netzwerkfehler");
        de.insert(
            TranslationKey::IdTooltip,
            "Teilen Sie diese ID mit dem Absender",
        );
        de.insert(
            TranslationKey::CompressTooltip,
            "Komprimierung für schnellere Übertragung aktivieren",
        );
        de.insert(
            TranslationKey::ThreadTooltip,
            "Mehr Threads = schnellere Übertragung",
        );
        de.insert(TranslationKey::FileName, "Dateiname");
        de.insert(TranslationKey::FileSize, "Dateigröße");
        de.insert(TranslationKey::FileCount, "Dateianzahl");
        de.insert(TranslationKey::TotalSize, "Gesamtgröße");
        de.insert(TranslationKey::AddMore, "Mehr hinzufügen");
        de.insert(TranslationKey::StartReceive, "Empfang starten");
        de.insert(TranslationKey::Sender, "Absender");
        de.insert(TranslationKey::Transferred, "Übertragen");
        de.insert(
            TranslationKey::EnableCompression,
            "LZ4-Komprimierung aktivieren",
        );
        de.insert(
            TranslationKey::SelectFilesHint,
            "oder Dateien hierher ziehen",
        );
        de.insert(TranslationKey::EnableEncryption, "Übertragungsverschlüsselung aktivieren");
        de.insert(TranslationKey::EnableAnimations, "Animationen aktivieren");
        de.insert(TranslationKey::ColorTheme, "Farbdesign");
        de.insert(TranslationKey::SaveSettings, "Einstellungen speichern");
        de.insert(TranslationKey::ElectricIndigoTheme, "Elektrisch Indigo");
        de.insert(TranslationKey::ClassicBlueTheme, "Klassisches Blau");
        de.insert(TranslationKey::DarkTheme, "Dunkles Design");
        de.insert(TranslationKey::LightTheme, "Helles Design");
        de.insert(TranslationKey::OrangeCatTheme, "Orangen-Tigerkatzenthema");
        data.insert(Language::German, de);

        // French (Français)
        let mut fr = HashMap::new();
        fr.insert(TranslationKey::AppTitle, "CatShare");
        fr.insert(
            TranslationKey::AppSubtitle,
            "Partage de fichiers rapide et facile",
        );
        fr.insert(TranslationKey::YourName, "Votre nom");
        fr.insert(TranslationKey::YourId, "Votre ID");
        fr.insert(
            TranslationKey::WaitingForConnection,
            "En attente de connexion...",
        );
        fr.insert(TranslationKey::Send, "Envoyer");
        fr.insert(TranslationKey::SendFile, "Envoyer un fichier");
        fr.insert(TranslationKey::SendFiles, "Envoyer des fichiers");
        fr.insert(TranslationKey::SelectFiles, "Sélectionner des fichiers");
        fr.insert(TranslationKey::SelectedFiles, "Fichiers sélectionnés");
        fr.insert(TranslationKey::EnterId, "Entrer l'ID");
        fr.insert(
            TranslationKey::EnterReceiverIdHint,
            "Entrez l'ID du destinataire",
        );
        fr.insert(
            TranslationKey::WaitingConfirmation,
            "En attente de confirmation...",
        );
        fr.insert(TranslationKey::SendingTo, "Envoi à");
        fr.insert(TranslationKey::Compress, "Compresser");
        fr.insert(TranslationKey::StartSend, "Démarrer l'envoi");
        fr.insert(TranslationKey::Receive, "Recevoir");
        fr.insert(TranslationKey::ReceiveFile, "Recevoir un fichier");
        fr.insert(TranslationKey::ReceiveFiles, "Recevoir des fichiers");
        fr.insert(
            TranslationKey::WaitingForSender,
            "En attente de l'expéditeur...",
        );
        fr.insert(TranslationKey::IncomingRequest, "Demande entrante");
        fr.insert(TranslationKey::Accept, "Accepter");
        fr.insert(TranslationKey::Reject, "Refuser");
        fr.insert(TranslationKey::ReceivingFrom, "Réception de");
        fr.insert(TranslationKey::SaveTo, "Enregistrer dans");
        fr.insert(TranslationKey::SelectFolder, "Sélectionner un dossier");
        fr.insert(TranslationKey::Progress, "Progression");
        fr.insert(TranslationKey::Compressing, "Compression...");
        fr.insert(TranslationKey::Transferring, "Transfert...");
        fr.insert(TranslationKey::Decompressing, "Décompression...");
        fr.insert(TranslationKey::Complete, "Terminé");
        fr.insert(TranslationKey::Failed, "Échoué");
        fr.insert(TranslationKey::Cancelled, "Annulé");
        fr.insert(TranslationKey::Speed, "Vitesse");
        fr.insert(TranslationKey::TimeRemaining, "Temps restant");
        fr.insert(TranslationKey::BytesTransferred, "Octets transférés");
        fr.insert(TranslationKey::Connected, "Connecté");
        fr.insert(TranslationKey::Disconnected, "Déconnecté");
        fr.insert(TranslationKey::Connecting, "Connexion...");
        fr.insert(TranslationKey::Error, "Erreur");
        fr.insert(TranslationKey::Settings, "Paramètres");
        fr.insert(TranslationKey::Language, "Langue");
        fr.insert(TranslationKey::Theme, "Thème");
        fr.insert(TranslationKey::DarkMode, "Mode sombre");
        fr.insert(TranslationKey::LightMode, "Mode clair");
        fr.insert(TranslationKey::CompressionLevel, "Niveau de compression");
        fr.insert(TranslationKey::ThreadCount, "Nombre de threads");
        fr.insert(TranslationKey::BufferSize, "Taille du tampon");
        fr.insert(TranslationKey::Cancel, "Annuler");
        fr.insert(TranslationKey::Confirm, "Confirmer");
        fr.insert(TranslationKey::Retry, "Réessayer");
        fr.insert(TranslationKey::Close, "Fermer");
        fr.insert(TranslationKey::Browse, "Parcourir");
        fr.insert(TranslationKey::Clear, "Effacer");
        fr.insert(TranslationKey::CopyId, "Copier l'ID");
        fr.insert(TranslationKey::Refresh, "Actualiser");
        fr.insert(TranslationKey::EditName, "Modifier le nom");
        fr.insert(TranslationKey::SaveName, "Enregistrer");
        fr.insert(TranslationKey::ConnectionFailed, "Échec de la connexion");
        fr.insert(TranslationKey::TransferComplete, "Transfert terminé !");
        fr.insert(TranslationKey::TransferFailed, "Échec du transfert");
        fr.insert(TranslationKey::InvalidId, "ID invalide");
        fr.insert(TranslationKey::NoFilesSelected, "Aucun fichier sélectionné");
        fr.insert(TranslationKey::RequestSent, "Demande envoyée");
        fr.insert(TranslationKey::RequestReceived, "Demande reçue");
        fr.insert(TranslationKey::RequestAccepted, "Demande acceptée");
        fr.insert(TranslationKey::RequestRejected, "Demande refusée");
        fr.insert(TranslationKey::FileNotFound, "Fichier non trouvé");
        fr.insert(TranslationKey::PermissionDenied, "Permission refusée");
        fr.insert(TranslationKey::NetworkError, "Erreur réseau");
        fr.insert(
            TranslationKey::IdTooltip,
            "Partagez cet ID avec l'expéditeur",
        );
        fr.insert(
            TranslationKey::CompressTooltip,
            "Activer la compression pour un transfert plus rapide",
        );
        fr.insert(
            TranslationKey::ThreadTooltip,
            "Plus de threads = transfert plus rapide",
        );
        fr.insert(TranslationKey::FileName, "Nom du fichier");
        fr.insert(TranslationKey::FileSize, "Taille du fichier");
        fr.insert(TranslationKey::FileCount, "Nombre de fichiers");
        fr.insert(TranslationKey::TotalSize, "Taille totale");
        fr.insert(TranslationKey::AddMore, "Ajouter plus");
        fr.insert(TranslationKey::StartReceive, "Démarrer la réception");
        fr.insert(TranslationKey::Sender, "Expéditeur");
        fr.insert(TranslationKey::Transferred, "Transféré");
        fr.insert(
            TranslationKey::EnableCompression,
            "Activer la compression LZ4",
        );
        fr.insert(
            TranslationKey::SelectFilesHint,
            "ou glissez-déposez des fichiers ici",
        );
        fr.insert(TranslationKey::EnableEncryption, "Activer le chiffrement des transferts");
        fr.insert(TranslationKey::EnableAnimations, "Activer les animations");
        fr.insert(TranslationKey::ColorTheme, "Thème de couleur");
        fr.insert(TranslationKey::SaveSettings, "Enregistrer les paramètres");
        fr.insert(TranslationKey::ElectricIndigoTheme, "Indigo Électrique");
        fr.insert(TranslationKey::ClassicBlueTheme, "Bleu Classique");
        fr.insert(TranslationKey::DarkTheme, "Thème Sombre");
        fr.insert(TranslationKey::LightTheme, "Thème Clair");
        fr.insert(TranslationKey::OrangeCatTheme, "Thème Chat Orange");
        data.insert(Language::French, fr);

        Self { data }
    }

    /// Get translation for a key in the specified language
    pub fn get(&self, lang: Language, key: TranslationKey) -> &'static str {
        self.data
            .get(&lang)
            .and_then(|m| m.get(&key))
            .copied()
            .unwrap_or_else(|| {
                // Fallback to English
                self.data
                    .get(&Language::English)
                    .and_then(|m| m.get(&key))
                    .copied()
                    .unwrap_or("???")
            })
    }
}

/// Initialize translations (call once at startup)
pub fn init() {
    TRANSLATIONS.get_or_init(Translations::new);
}

/// Get translation for a key
pub fn t(key: TranslationKey, lang: Language) -> &'static str {
    TRANSLATIONS
        .get()
        .map(|tr| tr.get(lang, key))
        .unwrap_or("???")
}

/// Shorthand macro for translations
#[macro_export]
macro_rules! tr {
    ($key:expr, $lang:expr) => {
        $crate::i18n::translations::t($key, $lang)
    };
}
