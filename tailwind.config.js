/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
        './src/**/*.rs',
    ],
    darkMode: 'class',
    theme: {
      extend: {
        flex: {
          '2': '2 2 0%',
          '3': '3 3 0%',
        },
        rotate: {
          '270': '270deg',
        }
      }
    },
    variants: {
      extend: {},
    },
    plugins: [
      require('@tailwindcss/forms'),
    ],
  }