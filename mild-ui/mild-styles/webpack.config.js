const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');

const STYLE_NAME = 'mild-style.css';

module.exports = (env) => {
  return {
    mode: env.production ? 'production' : 'development',
    entry: './index.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'bundle.js',
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
      new CopyPlugin({
        patterns: [
          {
            from: path.resolve(__dirname, `dist/${STYLE_NAME}`),
            to: path.resolve(
              __dirname,
              `../crates/docs/assets/css/${STYLE_NAME}`
            ),
          },
        ],
      }),
    ],
  };
};
