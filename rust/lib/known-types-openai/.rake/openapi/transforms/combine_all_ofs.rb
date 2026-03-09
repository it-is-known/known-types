# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

##
# Combines `allOf: [...]` schemas into a single schema.
#
# This should be done *after* the `DerefAliases` transformation.
module OpenAPI::Transforms::CombineAllOfs
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schemas!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      transform_schema!(schema, schemas)
    end
  end # transform_schemas!

  protected

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schema!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:allOf]
        variants = schema.delete(:allOf)
        raise ArgumentError, variants.inspect unless variants.is_a?(Array)
        raise ArgumentError, variants.inspect unless variants.all? { it.is_a?(Hash) && it[:'$ref'] }
        schema[:properties] ||= {}
        schema[:required] ||= []
        variants.each do |variant|
          target_ref = variant[:'$ref'].split('/').last.to_sym
          target = schemas[target_ref]
          raise ArgumentError, variants.inspect unless target[:type]&.to_sym == :object
          schema[:properties].merge!(target[:properties] || {})
          schema[:required] += target[:required] || []
        end
      else # nothing to do
    end
  end # transform_schema!
end # OpenAPI::Transforms::CombineAllOfs
