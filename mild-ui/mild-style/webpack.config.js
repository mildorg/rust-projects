const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const STYLE_NAME = 'style.css';
const OUT_FILE = 'bundle.js';

module.exports = (env) => {
  const isPro = !!env.production;
  const proDir = path.resolve(__dirname, 'dist');
  const devDir = path.resolve(__dirname, '../crates/docs/assets');

  return {
    mode: isPro ? 'production' : 'development',
    entry: './index.js',
    output: {
      path: isPro ? proDir : devDir,
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
      new MiniCssExtractPlugin({ filename: STYLE_NAME }),
      new CleanWebpackPlugin({
        dangerouslyAllowCleanPatternsOutsideProject: true,
        dry: false,
        cleanAfterEveryBuildPatterns: [
          isPro
            ? path.resolve(proDir, OUT_FILE)
            : path.resolve(devDir, OUT_FILE),
        ],
      }),
    ],
  };
};
