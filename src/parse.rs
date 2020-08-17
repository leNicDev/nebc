use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct MavenArtifact {
    repository_url: String,
    group_id: String,
    artifact_id: String,
    version: String,
}

// nebc artifact format:
// <repository url>:<group id>.<artifact id>@<tag|version>
// example: repo1.maven.org/maven2:org.springframework.boot.spring-boot-starter-web@latest
pub fn nebc_to_maven(nebc_artifact: &String) -> Result<MavenArtifact, Error> {
    // get version from nebc artifact
    let mut version: String;
    if nebc_artifact.contains("@") {
        version = nebc_artifact.split("@").collect::<Vec<&str>>()[1].to_string();
    } else {
        version = String::from("version");
    }

    // get repository url from nebc artifact
    let mut repository_url = String::new();
    if nebc_artifact.contains(":") {
        repository_url = nebc_artifact.split(":").collect::<Vec<&str>>()[0].to_string();
    } else {
        return Result::Err(Error::new(ErrorKind::Other, "No repository url defined"));
    }

    // get group id from nebc artifact
    let mut group_id = String::new();
    if nebc_artifact.contains(":") {
        group_id = nebc_artifact.split(":").collect::<Vec<&str>>()[1].to_string();

        // remove version/tag from group id
        if group_id.contains("@") {
            group_id = group_id.split("@").collect::<Vec<&str>>()[0].to_string();
        }

        // remove artifact id
        let split = group_id.split(".").collect::<Vec<&str>>();
        group_id = split[0..split.len() - 1].join(".");
    } else {
        return Result::Err(Error::new(ErrorKind::Other, "No group id defined"));
    }

    // get artifact id from nebc artifact
    let mut artifact_id = String::new();
    // remove version from nebc artifact
    if nebc_artifact.contains("@") {
        artifact_id = nebc_artifact.split("@").collect::<Vec<&str>>()[0].to_string();
    }
    if nebc_artifact.contains(".") {
        artifact_id = artifact_id.rsplit(".").collect::<Vec<&str>>()[0].to_string();
    } else {
        return Result::Err(Error::new(ErrorKind::Other, "Malformed nebc artifact"));
    }

    let maven_artifact = MavenArtifact {
        repository_url,
        group_id,
        artifact_id,
        version,
    };

    return Result::Ok(maven_artifact);
}

//pub fn maven_to_nebc(maven_artifact: &MavenArtifact) -> String {

//}