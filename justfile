#-------------------------Kubernetes Proto Generation---------------------------
KUBERNETES_VERSIONS := "1.30.1 1.31.1 1.32.1 1.33.0"
VERSION_PREFIX := "v"

default:
  @just --list

protos-dl:
    #!/usr/bin/env bash
    set -exuo pipefail
    mkdir -p k8s-pb
    cd k8s-pb
    for version in {{KUBERNETES_VERSIONS}}; do
        rm -rf "protos-${version}" && mkdir "protos-${version}" && cd "protos-${version}"
        for x in api apimachinery apiextensions-apiserver kube-aggregator metrics; do
            mkdir -p ./$x
            curl -sSL "https://github.com/kubernetes/$x/archive/refs/tags/kubernetes-${version}.tar.gz" | tar xzf - -C ./$x/ --strip-components=1
            fd -e proto -x sh -c "mkdir -p k8s.io/'{//}'; mv '{}' k8s.io/'{}'" ';' . ./$x
            # Rename directories in k8s.io namespace
            if [[ -d "k8s.io/apiextensions-apiserver" ]]; then
                mv "k8s.io/apiextensions-apiserver" "k8s.io/apiextensions_apiserver"
            fi
            
            if [[ -d "k8s.io/kube-aggregator" ]]; then
                mv "k8s.io/kube-aggregator" "k8s.io/kube_aggregator"
            fi
            rm -rf ./$x
        done
        cd ..
    done

protos-patch:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb
    for version in {{KUBERNETES_VERSIONS}}; do
        cd "protos-${version}"
        fd -e proto -x sd 'k8s\.io\.(.+);' '$1;' {}
        fd -e proto -x sd 'import "k8s\.io/(.+)";' 'import "$1";' {}
        mv k8s.io/* .
        rmdir k8s.io
        cd ..
    done

protos-rust:
    #!/usr/bin/env bash
    set -exuo pipefail
    cargo build -p k8s-rs-pb-codegen --release
    mv -f target/release/k8s-rs-pb-codegen ./codegen
    
    for version in {{KUBERNETES_VERSIONS}}; do
        version_dir="{{VERSION_PREFIX}}$(echo ${version} | tr . _ | cut -d'_' -f1-2)"
        proto_src="k8s-pb/protos-${version}"
        rust_dest="k8s-rs-pb/src/${version_dir}"
        
        # Создаем целевую директорию, если её нет
        mkdir -p "${rust_dest}"
        
        # Копируем proto-файлы в целевую директорию
        cp -R "${proto_src}"/* "${rust_dest}/"
        
        # Исправленный вызов с правильным порядком аргументов
        ./codegen overwrite false "${version_dir}"
        
        # Фикс путей
        cd "${rust_dest}"
        fd -e rs -x sd 'super::generated::ObjectMeta' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::ObjectMeta"
        #-----experimantal with time, cause it has same fields ------
        fd -e rs -x sd 'super::generated::Time' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Time"
        fd -e rs -x sd 'super::generated::MicroTime' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Time"
        fd -e rs -x sd 'super::generated::Timestamp' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Time"
        #------end experimental----------------
        fd -e rs -x sd 'super::generated::LabelSelectorAttributes' "crate::${version_dir}::api::authorization::v1::LabelSelectorAttributes"
        fd -e rs -x sd 'super::generated::ListMeta' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::ListMeta"
        fd -e rs -x sd 'super::generated::RawExtension' "crate::${version_dir}::apimachinery::pkg::runtime::RawExtension"
        fd -e rs -x sd 'super::generated::ObjectReference' "crate::${version_dir}::api::core::v1::ObjectReference"
        fd -e rs -x sd 'super::generated::Quantity' "crate::${version_dir}::apimachinery::pkg::api::resource::Quantity"
        fd -e rs -x sd 'super::generated::LabelSelector' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::LabelSelector"
        fd -e rs -x sd 'super::generated::PartialObjectMetadata' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::PartialObjectMetadata"
        fd -e rs -x sd 'super::generated::UserInfo' "crate::${version_dir}::api::authentication::v1::UserInfo"
        fd -e rs -x sd 'super::generated::Status' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Status"
        fd -e rs -x sd 'super::generated::Duration' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Duration"
        fd -e rs -x sd 'super::generated::GroupVersionKind' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::GroupVersionKind"
        fd -e rs -x sd 'super::generated::GroupVersionResource' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::GroupVersionResource"
        fd -e rs -x sd 'super::generated::IntOrString' "crate::${version_dir}::apimachinery::pkg::util::intstr::IntOrString"
        fd -e rs -x sd 'super::generated::Toleration' "crate::${version_dir}::api::core::v1::Toleration"
        fd -e rs -x sd 'super::generated::PodTemplateSpec' "crate::${version_dir}::api::core::v1::PodTemplateSpec"
        fd -e rs -x sd 'super::generated::TopologySelectorTerm' "crate::${version_dir}::api::core::v1::TopologySelectorTerm"
        fd -e rs -x sd 'super::generated::NodeSelector' "crate::${version_dir}::api::core::v1::NodeSelector"
        fd -e rs -x sd 'super::generated::EventSource' "crate::${version_dir}::api::core::v1::EventSource"
        fd -e rs -x sd 'super::generated::OwnerReference' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::OwnerReference"
        fd -e rs -x sd 'super::generated::Condition' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::Condition"
        fd -e rs -x sd 'super::generated::RuleWithOperations' "crate::${version_dir}::api::admissionregistration::v1::RuleWithOperations"
        fd -e rs -x sd 'super::generated::PersistentVolumeClaim' "crate::${version_dir}::api::core::v1::PersistentVolumeClaim"
        fd -e rs -x sd 'super::generated::TypedLocalObjectReference' "crate::${version_dir}::api::core::v1::TypedLocalObjectReference"
        fd -e rs -x sd 'super::generated::PersistentVolumeSpec' "crate::${version_dir}::api::core::v1::PersistentVolumeSpec"
        fd -e rs -x sd 'super::generated::DeleteOptions' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::DeleteOptions"
        fd -e rs -x sd 'super::generated::JobSpec' "crate::${version_dir}::api::batch::v1::JobSpec"
        fd -e rs -x sd 'super::generated::FieldSelectorAttributes' "crate::${version_dir}::api::authorization::v1::FieldSelectorAttributes"
        fd -e rs -x sd 'super::generated::FieldSelectorRequirement' "crate::${version_dir}::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement"
        cd ../../../
        
        ./codegen overwrite true "${version_dir}"
    done

protos-end:
    #!/usr/bin/env bash
    set -exuo pipefail
    rm -rf k8s-pb codegen

protos: protos-dl protos-patch protos-rust protos-end