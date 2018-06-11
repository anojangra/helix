pub fn sql() -> &'static str {
    "SELECT ticker
     FROM btcc.hourly
     GROUP BY ticker;"
}
