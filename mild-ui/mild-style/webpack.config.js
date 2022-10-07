const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const OUT_FILE = 'bundle.js';

module.exports = (env) => {
  const isPro = !!env.production;
  const proDist = path.resolve(__dirname, 'dist');
  const devDist = path.resolve(__dirname, '../crates/docs/assets/core-css');

  return {
    mode: isPro ? 'production' : 'development',
    entry: './index.js',
    output: {
      path: isPro ? proDist : devDist,
      filename: OUT_FILE,
      clean: true,
    },

    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [MiniCssExtractPlugin.loader, 'css-loader'],
        },
        {
          test: /\.s[ac]ss$/i,
          use: [MiniCssExtractPlugin.loader, 'css-loader', 'sass-loader'],
        },
      ],
    },

    optimization: {
      minimizer: [
        new CssMinimizerPlugin({
          minimizerOptions: {
            preset: [
              'default',
              {
                discardComments: { removeAll: true },
              },
            ],
          },
        }),
      ],
    },

    plugins: [
      new MiniCssExtractPlugin({
        filename: isPro ? 'style.min.css' : 'style.css',
      }),
      new CleanWebpackPlugin({
        dangerouslyAllowCleanPatternsOutsideProject: true,
        dry: false,
        cleanAfterEveryBuildPatterns: [
          isPro
            ? path.resolve(proDist, OUT_FILE)
            : path.resolve(devDist, OUT_FILE),
        ],
      }),
    ],
  };
};
