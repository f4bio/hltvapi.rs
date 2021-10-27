const path = require("path");
const { merge } = require("webpack-merge");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const common = require("./webpack.common.js");

const devConfig = {
  mode: "development",
  output: {
    path: path.resolve(__dirname, "web", "dist"),
    filename: "[name].js",
    clean: true,
    assetModuleFilename: "[name][ext]",
  },
  watchOptions: {
    ignored: /node_modules/,
  },
  devServer: {
    port: 31337,
    static: "./dist",
  },
  optimization: {
    splitChunks: false,
  },
  devtool: "inline-source-map",
  module: {
    rules: [
      {
        test: /\.(m?js|es6)$/,
        exclude: /(node_modules)/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env"],
            plugins: ["@babel/plugin-transform-runtime"],
          },
        },
        generator: {
          filename: "[name][ext]",
        },
      },
      {
        test: /\.html$/i,
        use: ["html-loader", "posthtml-loader"],
        generator: {
          filename: "[name][ext]",
        },
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
        generator: {
          filename: "[name][ext]",
        },
      },
    ],
  },
  plugins: [new MiniCssExtractPlugin()],
};

module.exports = merge(common, devConfig);
