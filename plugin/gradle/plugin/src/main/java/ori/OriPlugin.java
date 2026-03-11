package ori;

import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.JavaVersion;

import com.android.build.gradle.AppExtension;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.*;
import java.util.Map;

public class OriPlugin implements Plugin<Project> {
    @Override
    public void apply(Project project) {
        project.getPlugins().withId(
            "com.android.application",
            plugin -> configureAndroid(project)
        );
    }

    public void configureAndroid(Project project) {
        project.getDependencies().add(
            "implementation",
            "ori:activity:1.0.0"
        );

        try {
            CargoMetadata meta = new CargoMetadata(project);

            AppExtension android = project.getExtensions().getByType(AppExtension.class);

            android.compileSdkVersion(35);
            android.setNamespace(meta.namespace);

            android.getDefaultConfig().setApplicationId(meta.applicationId);
            android.getDefaultConfig().setMinSdk(21);
            android.getDefaultConfig().setTargetSdk(35);

            android.getCompileOptions().setSourceCompatibility(JavaVersion.VERSION_17);
            android.getCompileOptions().setTargetCompatibility(JavaVersion.VERSION_17);
        } catch (IOException e) {
            throw new RuntimeException("Failed to read cargo metadata", e);
        }
    }
}

class CargoMetadata {
    String namespace;
    String applicationId;

    public CargoMetadata(Project project) throws IOException {
        String metaString = readMetadata(project);
        ObjectMapper mapper = new ObjectMapper();
        JsonNode cargoMeta = mapper.readTree(metaString);

        JsonNode pkg = null;
        for (var p : cargoMeta.get("packages")) {
            String defaultMember = cargoMeta
                .get("workspace_default_members")
                .get(0)
                .asText();

            if (p.get("id").asText().equals(defaultMember)) {
                pkg = p;
            }
        }

        if (pkg == null) {
            throw new RuntimeException("No default cargo workspaces defined");
        }

        JsonNode meta = pkg.get("metadata").get("android");

        if (meta == null) {
            throw new RuntimeException("Android metadata not found in `Cargo.toml`");
        }

        namespace = meta.get("package").asText();
        applicationId = meta.get("package").asText();
    }

    private static String readMetadata(Project project) throws IOException {
        ProcessBuilder processBuilder = new ProcessBuilder(
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--no-deps"
        );

        processBuilder.directory(project.getRootDir());
        processBuilder.redirectErrorStream(true);

        Process process = processBuilder.start();

        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));

        String line;
        StringBuilder output = new StringBuilder();
        while ((line = reader.readLine()) != null) output.append(line);

        try {
            process.waitFor();
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }

        return output.toString();
    }
}
