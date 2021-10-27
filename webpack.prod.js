const path = require("path");
const { merge } = require("webpack-merge");
const TerserPlugin = require("terser-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WebpackPwaManifest = require("webpack-pwa-manifest");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const ImageMinimizerPlugin = require("image-minimizer-webpack-plugin");
const WorkboxPlugin = require("workbox-webpack-plugin");
const common = require("./webpack.common.js");

const prodConfig = {
  mode: "production",
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin(), new CssMinimizerPlugin()],
    splitChunks: {
      chunks: "all",
      cacheGroups: {
        commons: {
          test: /node_modules/,
          chunks: "initial",
          filename: "vendors.[contenthash].js",
          priority: 1,
          maxInitialRequests: 2,
          minChunks: 3, // count of entries
        },
      },
    },
  },
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
      },
      {
        test: /\.html$/i,
        use: [
          {
            loader: "html-loader",
            options: { minimize: true },
          },
          {
            loader: "posthtml-loader",
          },
        ],
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
    ],
  },
  plugins: [
    new ImageMinimizerPlugin({
      severityError: "warning",
      minimizerOptions: {
        plugins: ["gifsicle", "jpegtran", "optipng"],
      },
    }),
    new MiniCssExtractPlugin(),
    new WebpackPwaManifest({
      name: "hltv.org API",
      short_name: "hltvapi",
      description: "Unofficial hltv.org API",
      background_color: "#3367D6",
      start_url: "/",
      display: "standalone",
      scope: "/",
      theme_color: "#3367D6",
      icons: [
        {
          src: path.resolve(__dirname, "web", "images", "icon.png"),
          size: "400x400",
          type: "image/png",
        },
        {
          src: path.resolve(
            __dirname,
            "web",
            "images",
            "android-chrome-192x192.png",
          ),
          size: "192x192",
          type: "image/png",
        },
        {
          src: path.resolve(
            __dirname,
            "web",
            "images",
            "android-chrome-384x384.png",
          ),
          size: "384x384",
          type: "image/png",
        },
      ],
    }),
  ],
};

module.exports = merge(common, prodConfig);
