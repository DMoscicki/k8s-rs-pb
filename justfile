#-------------------------Inspired by kube-rs/k8s-pb------------------------------

default:
  @just --list

# renovate: datasource=github-releases depName=kubernetes/kubernetes
KUBERNETES_VERSION := "1.30.1"

protos-dl:
	#!/usr/bin/env bash
	set -exuo pipefail
	mkdir -p k8s-pb
	cd k8s-pb
	rm -rf protos && mkdir protos && cd protos
	for x in api apimachinery apiextensions-apiserver kube-aggregator metrics; do
		mkdir -p ./$x
		curl -sSL https://github.com/kubernetes/$x/archive/refs/tags/kubernetes-{{KUBERNETES_VERSION}}.tar.gz | tar xzf - -C ./$x/ --strip-components=1
		fd -e proto -x sh -c "mkdir -p k8s.io/'{//}'; mv '{}' k8s.io/'{}'" ';' . ./$x
		rm -rf ./$x
	done

# Patch protos schemas to fix import paths
protos-patch:
	#!/usr/bin/env bash
	set -exuo pipefail
	cd k8s-pb
	fd -e proto -x sd 'k8s\.io\.(.+);' '$1;' {}
	fd -e proto -x sd 'import "k8s\.io/(.+)";' 'import "$1";' {}
	mv protos/k8s.io/* protos/
	rmdir protos/k8s.io/

# Generate protos path list for prost
protos-list:
	#!/usr/bin/env bash
	set -exuo pipefail
	cd k8s-pb
	sort --version
	fd -e proto | sort -fd > protos.list

# Generate Rust files 
# just protos-rust PATH_TO_FOLDER
protos-rust:
	#!/usr/bin/env bash
	set -exuo pipefail
	mkdir -p k8s-pb/src
	cargo build -p k8s-rs-pb-codegen --release; rm -rf ./codegen; mv -f target/release/k8s-rs-pb-codegen ./codegen;
	rm -rf k8s-rs-pb/src/*/;
	mv -f k8s-pb/protos/* k8s-rs-pb/src/;
	./codegen overwrite false;
	cd k8s-rs-pb/src;
	#fd generated.rs -x mv {} {//}/mod.rs;
	fd -e rs -x sd 'super::generated::ObjectMeta' 'crate::apimachinery::pkg::apis::meta::v1::ObjectMeta'
	#-----experimantal with time, cause it has same fields ------
	fd -e rs -x sd 'super::generated::Time' 'crate::apimachinery::pkg::apis::meta::v1::Time'
	fd -e rs -x sd 'super::generated::MicroTime' 'crate::apimachinery::pkg::apis::meta::v1::Time'
	fd -e rs -x sd 'super::generated::Timestamp' 'crate::apimachinery::pkg::apis::meta::v1::Time'
	#------end experimental----------------
	fd -e rs -x sd 'super::generated::ListMeta' 'crate::apimachinery::pkg::apis::meta::v1::ListMeta'
	fd -e rs -x sd 'super::generated::RawExtension' 'crate::apimachinery::pkg::runtime::RawExtension'
	fd -e rs -x sd 'super::generated::ObjectReference' 'crate::api::core::v1::ObjectReference'
	fd -e rs -x sd 'super::generated::Quantity' 'crate::apimachinery::pkg::api::resource::Quantity'
	fd -e rs -x sd 'super::generated::LabelSelector' 'crate::apimachinery::pkg::apis::meta::v1::LabelSelector'
	fd -e rs -x sd 'super::generated::PartialObjectMetadata' 'crate::apimachinery::pkg::apis::meta::v1::PartialObjectMetadata'
	fd -e rs -x sd 'super::generated::UserInfo' 'crate::api::authentication::v1::UserInfo'
	fd -e rs -x sd 'super::generated::Status' 'crate::apimachinery::pkg::apis::meta::v1::Status'
	fd -e rs -x sd 'super::generated::Duration' 'crate::apimachinery::pkg::apis::meta::v1::Duration'
	fd -e rs -x sd 'super::generated::GroupVersionKind' 'crate::apimachinery::pkg::apis::meta::v1::GroupVersionKind'
	fd -e rs -x sd 'super::generated::GroupVersionResource' 'crate::apimachinery::pkg::apis::meta::v1::GroupVersionResource'
	fd -e rs -x sd 'super::generated::IntOrString' 'crate::apimachinery::pkg::util::intstr::IntOrString'
	fd -e rs -x sd 'super::generated::Toleration' 'crate::api::core::v1::Toleration'
	fd -e rs -x sd 'super::generated::PodTemplateSpec' 'crate::api::core::v1::PodTemplateSpec'
	fd -e rs -x sd 'super::generated::TopologySelectorTerm' 'crate::api::core::v1::TopologySelectorTerm'
	fd -e rs -x sd 'super::generated::NodeSelector' 'crate::api::core::v1::NodeSelector'
	fd -e rs -x sd 'super::generated::EventSource' 'crate::api::core::v1::EventSource'
	fd -e rs -x sd 'super::generated::OwnerReference' 'crate::apimachinery::pkg::apis::meta::v1::OwnerReference'
	fd -e rs -x sd 'super::generated::Condition' 'crate::apimachinery::pkg::apis::meta::v1::Condition'
	fd -e rs -x sd 'super::generated::RuleWithOperations' 'crate::api::admissionregistration::v1::RuleWithOperations'
	fd -e rs -x sd 'super::generated::PersistentVolumeClaim' 'crate::api::core::v1::PersistentVolumeClaim'
	fd -e rs -x sd 'super::generated::TypedLocalObjectReference' 'crate::api::core::v1::TypedLocalObjectReference'
	fd -e rs -x sd 'super::generated::PersistentVolumeSpec' 'crate::api::core::v1::PersistentVolumeSpec'
	fd -e rs -x sd 'super::generated::DeleteOptions' 'crate::apimachinery::pkg::apis::meta::v1::DeleteOptions'
	fd -e rs -x sd 'super::generated::JobSpec' 'crate::api::batch::v1::JobSpec'
	cd ..; cd ..; ./codegen overwrite true; rm -rf k8s-rs-pb/src/mod.rs;

protos-end:
	#!/usr/bin/env bash
	set -exuo pipefail
	mv -f k8s-rs-pb/src/apiextensions-apiserver k8s-rs-pb/src/apiextensions_apiserver;
	mv -f k8s-rs-pb/src/kube-aggregator k8s-rs-pb/src/kube_aggregator;
	rm -rf k8s-pb

# Download and generate all protos dependent files
protos: protos-dl protos-patch protos-list protos-rust protos-end