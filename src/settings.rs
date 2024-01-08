pub const FRONTEND_HOST: &str = if cfg!(debug_assertions) {
    "http://localhost:3000"
} else {
    "https://frontend-junhsss.vercel.app"
};
