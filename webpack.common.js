const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WorkboxPlugin = require("workbox-webpack-plugin");

module.exports = {
  entry: path.resolve(__dirname, "web", "index.js"),
  output: {
    path: path.resolve(__dirname, "web", "dist"),
    publicPath: "/",
    chunkFilename: "[chunkhash].js",
    filename: "[contenthash].js",
    assetModuleFilename: "[contenthash][ext][query]",
  },
  module: {
    rules: [
      {
        test: /redoc.standalone.js$/i,
        type: "asset",
        generator: {
          filename: "[name][ext][query]",
        },
      },
      {
        test: /\.(jpe?g|png|gif)$/i,
        type: "asset",
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset",
      },
    ],
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: path.resolve(__dirname, "openapi.yaml") },
        { from: path.resolve(__dirname, "web", "images", "logo.png") },
      ],
    }),
    new WorkboxPlugin.GenerateSW(),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "web", "templates", "base.tera.html"),
      filename: "base.tera.html",
      favicon: path.resolve(__dirname, "web", "images", "icon.png"),
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "web", "templates", "docs.tera.html"),
      filename: "docs.tera.html",
      inject: false,
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(
        __dirname,
        "web",
        "templates",
        "calendar.tera.html",
      ),
      filename: "calendar.tera.html",
      inject: false,
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(
        __dirname,
        "web",
        "templates",
        "landing.tera.html",
      ),
      filename: "landing.tera.html",
      inject: false,
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "web", "templates", "news.tera.html"),
      filename: "news.tera.html",
      inject: false,
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(
        __dirname,
        "web",
        "templates",
        "snackbar.tera.html",
      ),
      filename: "snackbar.tera.html",
      inject: false,
    }),
  ],
};
