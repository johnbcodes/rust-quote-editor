module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        'header': 'hsl(var(--color-text-header) / <alpha-value>)',
        'body': 'hsl(var(--color-text-body) / <alpha-value>)',
        'primary': 'hsl(var(--color-primary) / <alpha-value>)',
        'primary-rotate': 'hsl(var(--color-primary-rotate) / <alpha-value>)',
        'primary-bg': 'hsl(var(--color-primary-bg) / <alpha-value>)',
        'secondary': 'hsl(var(--color-secondary) / <alpha-value>)',
        'secondary-rotate': 'hsl(var(--color-secondary-rotate) / <alpha-value>)',
        'tertiary': 'hsl(var(--color-tertiary) / <alpha-value>)',
        'glint': 'hsl(var(--color-glint) / <alpha-value>)',
        'background': 'hsl(var(--color-background) / <alpha-value>)',
        'light': 'hsl(var(--color-light) / <alpha-value>)',
        'dark': 'hsl(var(--color-dark) / <alpha-value>)',
      },
      animation: {
        'appear-then-fade': 'appear-then-fade 4s both',
      },
      keyframes: {
        'appear-then-fade': {
          '0%, 100%': { opacity: '0' },
          '5%, 60%': { opacity: '1' },
        }
      }
    },
  }
}
