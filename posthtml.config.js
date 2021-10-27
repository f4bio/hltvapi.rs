require("toml-require").install();

module.exports = (ctx) => ({
  ...ctx.options,
  plugins: {
    "posthtml-expressions": {
      locals: {
        news_xml_path: "/api/news/feed.xml",
        calendar_ics_path: "/api/fixture/calendar.ics",
        app_build_version: require("./Cargo.toml").package.version,
        app_build_datetime: new Date().toISOString(),
      },
    },
  },
});
