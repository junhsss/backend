pub const FRONTEND_HOST: &str = if cfg!(debug_assertions) {
    "http://localhost:3000"
} else {
    "https://frontend-junhsss.vercel.app"
};

pub const AUTH_SECRET: &str = if cfg!(debug_assertions) {
    "TZjmZvqsiMCYGNbWl3msQtuVjBbo51V9"
} else {
    "s3VMfQ3DgdSlKQodapOnFdAowsH4rViJ"
};

pub const TIMEOUT: i64 = 3600;
