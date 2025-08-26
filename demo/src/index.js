import { codesnap } from "@pyoner/codesnap";

// Get references to HTML elements
const codeInput = document.getElementById("code-input");
const languageInput = document.getElementById("language-input");
const themeInput = document.getElementById("theme-input");
const backgroundInput = document.getElementById("background-input");
const scaleFactorInput = document.getElementById("scale-factor-input");
const generateButton = document.getElementById("generate-button");
const imageOutput = document.getElementById("image-output");
const downloadLink = document.getElementById("download-link");

// Set initial values for inputs
codeInput.value = `function greet(name) {
  console.log(\`Hello, \${name}!\`);
}`;
languageInput.value = "javascript";
themeInput.value = "dracula";
backgroundInput.value = "#282A36";
scaleFactorInput.value = "2";

// Function to generate and display the code snapshot
function generateSnapshot() {
  const code = codeInput.value;
  const language = languageInput.value;
  const theme = themeInput.value;
  const background = backgroundInput.value;
  const scaleFactor = parseFloat(scaleFactorInput.value);

  try {
    const image = codesnap(code, language, {
      theme: theme,
      background: background,
      scale_factor: scaleFactor,
    });

    const blob = new Blob([image.data], { type: "image/png" });
    const imageUrl = URL.createObjectURL(blob);

    imageOutput.src = imageUrl;
    imageOutput.style.display = "block";

    downloadLink.href = imageUrl;
    downloadLink.download = `codesnap-${Date.now()}.png`;
    downloadLink.style.display = "block";
  } catch (error) {
    console.error("Error generating snapshot:", error);
    alert("Failed to generate snapshot. Check console for details.");
    imageOutput.style.display = "none";
    downloadLink.style.display = "none";
  }
}

// Attach event listener to the button
generateButton.addEventListener("click", generateSnapshot);

// Generate initial snapshot on page load
generateSnapshot();
