const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const OUT_FILE = 'bundle.js';

module.exports = (env) => {
  const isPro = !!env.production;
  const dist = path.resolve(__dirname, 'dist');

  return {
    mode: isPro ? 'production' : 'development',
    entry: './index.js',
    output: {
      path: dist,
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
        filename: 'style.css',
      }),
      new CleanWebpackPlugin({
        dangerouslyAllowCleanPatternsOutsideProject: true,
        dry: false,
        cleanAfterEveryBuildPatterns: [path.resolve(dist, OUT_FILE)],
      }),
    ],
  };
};
